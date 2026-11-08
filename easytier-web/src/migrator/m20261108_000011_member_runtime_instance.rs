use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE network_members ADD COLUMN runtime_instance_id TEXT;
                 CREATE UNIQUE INDEX idx_member_runtime_instance
                 ON network_members (user_id, device_id, runtime_instance_id)
                 WHERE runtime_instance_id IS NOT NULL",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP INDEX idx_member_runtime_instance;
                 ALTER TABLE network_members DROP COLUMN runtime_instance_id",
            )
            .await?;
        Ok(())
    }
}
