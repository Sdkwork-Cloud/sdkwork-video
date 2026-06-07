pub fn video_generation_repository_contract_methods() -> Vec<&'static str> {
    vec![
        "create_generation",
        "mark_provider_submitted",
        "upsert_provider_task",
        "record_provider_webhook_event",
        "upsert_generation_outputs",
        "mark_drive_importing",
        "mark_drive_imported",
        "mark_generation_succeeded",
        "mark_generation_failed",
        "enqueue_notification",
        "find_due_provider_tasks",
        "find_pending_drive_imports",
    ]
}
