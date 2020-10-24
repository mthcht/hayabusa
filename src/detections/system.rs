use crate::models::event;
use std::collections::HashMap;

pub struct System {}

impl System {
    pub fn new() -> System {
        System {}
    }

    pub fn detection(
        &mut self,
        event_id: String,
        system: &event::System,
        event_data: HashMap<String, String>,
    ) {
        self.system_log_clear(&event_id);
        self.windows_event_log(&event_id, event_data);
        self.new_service_created(&event_id);
        self.interactive_service_warning(&event_id);
        self.suspicious_service_name(&event_id);
    }

    fn new_service_created(&mut self, event_id: &String) {
        if event_id != "7045" {
            return
        }

        println!("Message : System Log Clear");
        println!("Results : The System log was cleared.");
    }

    fn interactive_service_warning(&mut self, event_id: &String) {
        if event_id != "7030" {
            return
        }

        println!("Message : System Log Clear");
        println!("Results : The System log was cleared.");
    }

    fn suspicious_service_name(&mut self, event_id: &String) {
        if event_id != "7036" {
            return
        }

        println!("Message : System Log Clear");
        println!("Results : The System log was cleared.");
    }

    fn system_log_clear(&mut self, event_id: &String) {
        if event_id != "104" {
            return
        }

        println!("Message : System Log Clear");
        println!("Results : The System log was cleared.");
    }

    fn windows_event_log(&mut self, event_id: &String, event_data: HashMap<String, String>) {
        if event_id != "7040" {
            return
        }

        if let Some(_param1) = event_data.get("param1") {
            if _param1 == "Windows Event Log" {
                println!("Service name : {}", _param1);
                if let Some(_param2) = event_data.get("param2") {
                    if _param2 == "disabled" {
                        println!("Message : Event Log Service Stopped");
                        println!(
                            "Results : Selective event log manipulation may follow this event."
                        );
                    } else if _param2 == "auto start" {
                        println!("Message : Event Log Service Started");
                        println!(
                            "Results : Selective event log manipulation may precede this event."
                        );
                    }
                }
            }
        }
    }
}
