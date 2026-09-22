CREATE TABLE "device_status_logs" (
    "device_id" INTEGER NOT NULL,
    "timestamp" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    PRIMARY KEY ("device_id", "timestamp")
);
-- #[toasty::breakpoint]
CREATE INDEX "index_device_status_logs_by_timestamp" ON "device_status_logs" ("timestamp");
-- #[toasty::breakpoint]
CREATE TABLE "cluster_item_ids" (
    "id" BLOB NOT NULL,
    "item_device" INTEGER NOT NULL,
    "item_endpoint" INTEGER NOT NULL,
    "item_cluster" INTEGER NOT NULL,
    "item_item" INTEGER NOT NULL,
    "item_item_is_attr" BOOLEAN NOT NULL,
    PRIMARY KEY ("id")
);
-- #[toasty::breakpoint]
CREATE UNIQUE INDEX "index_cluster_item_ids_by_item_device_and_item_endpoint_and_item_cluster_and_item_item_and_item_item_is_attr" ON "cluster_item_ids" ("item_device", "item_endpoint", "item_cluster", "item_item", "item_item_is_attr");
-- #[toasty::breakpoint]
CREATE TABLE "event_logs" (
    "event_id" BLOB NOT NULL,
    "timestamp" TEXT NOT NULL,
    "value" BLOB NOT NULL,
    PRIMARY KEY ("event_id", "timestamp")
);
-- #[toasty::breakpoint]
CREATE INDEX "index_event_logs_by_timestamp" ON "event_logs" ("timestamp");
-- #[toasty::breakpoint]
CREATE TABLE "attribute_changes" (
    "attribute_id" BLOB NOT NULL,
    "timestamp" TEXT NOT NULL,
    "value" BLOB NOT NULL,
    PRIMARY KEY ("attribute_id", "timestamp")
);
-- #[toasty::breakpoint]
CREATE INDEX "index_attribute_changes_by_timestamp" ON "attribute_changes" ("timestamp");
