/*
 * Copyright (c) 2023 Stalwart Labs Ltd.
 *
 * This file is part of the Stalwart Mail Server.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 * in the LICENSE file at the top-level directory of this distribution.
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * You can be released from the requirements of the AGPLv3 license by
 * purchasing a commercial license. Please contact licensing@stalw.art
 * for more details.
*/

use mysql_async::{prelude::Queryable, OptsBuilder, Pool, PoolConstraints, PoolOpts};

use crate::{
    SUBSPACE_BITMAPS, SUBSPACE_BLOBS, SUBSPACE_BLOB_DATA, SUBSPACE_COUNTERS, SUBSPACE_INDEXES,
    SUBSPACE_INDEX_VALUES, SUBSPACE_LOGS, SUBSPACE_VALUES,
};

use super::MysqlStore;

impl MysqlStore {
    pub async fn open(config: &utils::config::Config) -> crate::Result<Self> {
        let mut opts = OptsBuilder::default()
            .ip_or_hostname(config.value_require("store.db.host")?.to_string())
            .user(config.value("store.db.user").map(|s| s.to_string()))
            .pass(config.value("store.db.password").map(|s| s.to_string()))
            .db_name(
                config
                    .value_require("store.db.database")?
                    .to_string()
                    .into(),
            )
            .wait_timeout(config.property("store.db.timeout")?);
        if let Some(port) = config.property("store.db.port")? {
            opts = opts.tcp_port(port);
        }

        // Configure connection pool
        let mut pool_min = PoolConstraints::default().min();
        let mut pool_max = PoolConstraints::default().max();
        if let Some(n_size) = config.property::<usize>("store.db.pool.min-connections")? {
            pool_min = n_size;
        }
        if let Some(n_size) = config.property::<usize>("store.db.pool.max-connections")? {
            pool_max = n_size;
        }
        opts = opts.pool_opts(
            PoolOpts::default().with_constraints(PoolConstraints::new(pool_min, pool_max).unwrap()),
        );

        let db = Self {
            conn_pool: Pool::new(opts),
        };

        db.create_tables().await?;

        Ok(db)
    }

    pub(super) async fn create_tables(&self) -> crate::Result<()> {
        let mut conn = self.conn_pool.get_conn().await?;

        for table in [SUBSPACE_VALUES, SUBSPACE_LOGS, SUBSPACE_INDEX_VALUES] {
            let table = char::from(table);
            conn.query_drop(&format!(
                "CREATE TABLE IF NOT EXISTS {table} (
                    k TINYBLOB,
                    v MEDIUMBLOB NOT NULL,
                    PRIMARY KEY (k(255))
                ) ENGINE=InnoDB"
            ))
            .await?;
        }

        conn.query_drop(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                k TINYBLOB,
                v LONGBLOB NOT NULL,
                PRIMARY KEY (k(255))
            ) ENGINE=InnoDB",
            char::from(SUBSPACE_BLOB_DATA),
        ))
        .await?;

        for table in [SUBSPACE_INDEXES, SUBSPACE_BITMAPS] {
            let table = char::from(table);
            conn.query_drop(&format!(
                "CREATE TABLE IF NOT EXISTS {table} (
                    k BLOB,
                    PRIMARY KEY (k(400))
                ) ENGINE=InnoDB"
            ))
            .await?;
        }

        for table in [SUBSPACE_BLOBS] {
            let table = char::from(table);
            conn.query_drop(&format!(
                "CREATE TABLE IF NOT EXISTS {table} (
                    k TINYBLOB,
                    PRIMARY KEY (k(255))
                ) ENGINE=InnoDB"
            ))
            .await?;
        }

        conn.query_drop(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                k TINYBLOB,
                v BIGINT NOT NULL DEFAULT 0,
                PRIMARY KEY (k(255))
            ) ENGINE=InnoDB",
            char::from(SUBSPACE_COUNTERS)
        ))
        .await?;

        Ok(())
    }
}
