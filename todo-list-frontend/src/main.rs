use std::collections::BTreeMap;

use eframe::{App, egui};
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use todo_list_common::TodoItem;

#[derive(Debug, Clone)]
struct ShowWindowData {
    show_window: bool,
    working_data: Option<TodoItem>,
    working_data_id: Option<u32>
}

#[derive(Debug, Clone)]
struct TodoApp {
    todo_entries: BTreeMap<u32, TodoItem>,
    first_run: bool,
    show_window_data: ShowWindowData
}

impl App for TodoApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tasks Todo:");
            ui.separator();

            if ui.button("⟳ Refresh Table").clicked() || self.first_run == true {
                refresh_entities(&mut self.todo_entries, &mut self.first_run);
            }

            // Render table
            render_table(ui, &mut self.todo_entries, &mut self.show_window_data);

            // Render window for task if user is editing the data
            if let Some(working_id) = self.show_window_data.working_data_id {
                if let Some(working_data) = self.show_window_data.working_data.as_mut() {
                    egui::Window::new(format!("Task {}", working_id))
                        .resizable(true)
                        .show(ctx, |ui| {
                            render_task_window(ui, &working_id, working_data);
                        });
                }
            }

        
        });
    }
}

fn refresh_entities (
    todo_entries: &mut BTreeMap<u32, TodoItem>,
    first_run: &mut bool
) {

    // Blocking request
    match reqwest::blocking::get("http://127.0.0.1:8081/list") {
        Ok(resp) => match resp.json::<BTreeMap<String, TodoItem>>() {
            Ok(items_map) => {
                todo_entries.clear();
                for (id_str, item) in items_map {
                    if let Ok(id) = id_str.parse::<u32>() {
                        todo_entries.insert(id, item);
                    } else {
                        eprintln!("Invalid ID in response: {}", id_str);
                    }
                }
                *first_run = false;
            }
            Err(err) => {
                eprintln!("Failed to parse JSON: {:?}", err);
            }
        },
        Err(err) => {
            eprintln!("HTTP request failed: {:?}", err);
        }
    }
}

fn render_table(
    ui: &mut Ui,
    todo_entries: &mut BTreeMap<u32, TodoItem>,
    show_window: &mut ShowWindowData
) {
    let tasks_table = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .columns(Column::auto(), 7)
        .header(20.0, |mut header| {
            header.col(|col| { col.strong("Done"); });
            header.col(|col| { col.strong("ID"); });
            header.col(|col| { col.strong("Title"); });
            header.col(|col| { col.strong("Description"); });
            header.col(|col| { col.strong("Due Date"); });
            header.col(|col| { col.strong("Created On"); });
            header.col(|col| { col.strong("View Details"); });
        })
        .body(|mut body| {
            for (id, item) in todo_entries {
                body.row(20.0, |mut row| {
                    row.col(|ui| { ui.label(if item.completed { "✅" } else { "❌" }); });
                    row.col(|ui| { ui.label(id.to_string()); });
                    row.col(|ui| { ui.label(&item.title); });
                    row.col(|ui| { ui.label(&item.description); });
                    row.col(|ui| { ui.label(item.due_date.to_rfc3339()); });
                    row.col(|ui| { ui.label(item.created_at.to_rfc3339()); });
                    row.col(|ui| { 

                        if ui.button("View Details").clicked() {
                            *show_window = ShowWindowData { show_window: true, working_data: Some(item.clone()), working_data_id: Some(id.clone()) }
                        }
                        
                    });
                });
            }
        });
}

fn render_task_window(
    ui: &mut Ui,
    working_id: &u32,
    working_data: &mut TodoItem
) {
    ui.vertical(|ui| {
        ui.label("Title:");
        ui.text_edit_singleline(&mut working_data.title);

        ui.separator();

        ui.label("Description:");
        ui.text_edit_multiline(&mut working_data.description);

        ui.separator();

        ui.label("Due Date (ISO 8601):");
        let mut due_date_str = working_data.due_date.to_rfc3339();
        if ui.text_edit_singleline(&mut due_date_str).changed() {
            if let Ok(parsed) = due_date_str.parse::<chrono::DateTime<chrono::Utc>>() {
                working_data.due_date = parsed;
            }
        }

        ui.separator();

        ui.label("Created At (ISO 8601):");
        let mut created_at_str = working_data.created_at.to_rfc3339();
        if ui.text_edit_singleline(&mut created_at_str).changed() {
            if let Ok(parsed) = created_at_str.parse::<chrono::DateTime<chrono::Utc>>() {
                working_data.created_at = parsed;
            }
        }

        ui.separator();

        ui.checkbox(&mut working_data.completed, "Completed");
    });
}


fn main() -> eframe::Result<()> {

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Todo List",
        options,
        Box::new(|_cc| Ok(Box::new(TodoApp {
            todo_entries: BTreeMap::new(),
            first_run: true,
            show_window_data: ShowWindowData { show_window: false, working_data: Option::None, working_data_id: Option::None }
        }))),
    )
}