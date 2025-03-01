use std::fs::{self, File};
use std::io::{self,Read,Write};
use std::path::Path;
use std::collections::HashMap;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum Status {
    TODO,
    PROGRESS,
    DONE,
}

impl Status {
    fn convert_string_to_status(s:&str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "TODO" => Ok(Status::TODO),
            "PROGRESS" => Ok(Status::PROGRESS),
            "DONE" => Ok(Status::DONE),
            _ => Err(format!("Invalid status not supported: {}", s))
    }
}
}

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Ticket {
    id: String,
    title: String,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    assignee: Option<String>,
}



#[derive(Debug,Serialize,Deserialize)]
struct TicketStore {
    tickets: HashMap<String, Ticket>,
}

impl TicketStore {
    fn new() -> Self {
        TicketStore{
            tickets: HashMap::new(),
        } 
    }
    
    fn load(path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        let ticket_store = serde_json::from_str(&file_content)? ;
        Ok(ticket_store)
    }

    fn save(&self, path: &Path) -> io::Result<()> {

        //TODO : test , prob does not work if parent Dir does not exist
        let mut file = File::create(path)?;
        let file_content = serde_json::to_string_pretty(self)?;
        file.write_all(file_content.as_bytes())?;
        Ok(())
    }

    fn add(&mut self, ticket: Ticket) {
        self.tickets.insert(ticket.id.clone(), ticket);
    }

    fn list_tickets(&self) -> Vec<&Ticket> {
        self.tickets.values().collect()
    }

    fn get_ticket(&mut self , id:&str) -> Option<&mut Ticket> {
        self.tickets.get_mut(id)
    }
}

#[derive(Parser)]
#[clap(name = "Ticket System")]
#[clap(author= "0xApplepie")]
#[clap(version = "1.0")]
#[clap(about = "A simple JIRA likeCLI for managing tickets", long_about = "This is a simple CLI for managing tickets. It allows you to create, list, and view tickets.")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    Create{
        #[clap(short, long)]
        title: String,

        #[clap(short, long)]
        description: String,

        #[clap(short, long)]
        assignee: Option<String>
    },

    List,

    ViewTicket {
        id: String,
    },

    Update {
        id: String,
        
        #[clap(short, long)]
        title: Option<String>,

        #[clap(short, long)]
        description: Option<String>,

        #[clap(short, long)]
        status: Option<String>,

        #[clap(short, long)]
        assignee: Option<String>,
    },
    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let path = Path::new("data/tickets.json");
    let cli = Cli::parse();

    let mut ticket_store = TicketStore::load(path)?;

    match cli.command {
        Commands::Create { title, description,assignee}=> {
            let now = Utc::now();
            let ticket = Ticket {
                id: Uuid::new_v4().to_string(),
                title,
                description,
                status: Status::TODO,
                created_at: now,
                updated_at: now,
                assignee,
            };
            let ticket_id = ticket.id.clone();
            ticket_store.add(ticket);
            ticket_store.save(path)?;
            println!("Ticket created successfully, with ID {}", ticket_id);
            Ok(())
        }

        Commands::List => {
            let tickets = ticket_store.list_tickets();
            if tickets.is_empty() {
                println!("No tickets found.");
                return Ok(());
            }
            
            println!("ID | Title | Status | Assignee");
            println!("------------------------");
            
            for ticket in tickets {
                let assignee = ticket.assignee.as_deref().unwrap_or("Unassigned");
                println!("{} | {} | {:?} | {}", 
                    ticket.id, ticket.title, ticket.status, assignee);
            }
            Ok(())
        }

        Commands::ViewTicket { id } => {
            let ticket = ticket_store.get_ticket(&id);
            match ticket {
                Some(ticket) => println!("{:?}", ticket),
                None => println!("Ticket not found."),
            }
            Ok(())
        }

        Commands::Update { id, title, description, status, assignee } => {
            let mut ticket = ticket_store.get_ticket(&id).unwrap();
            if let Some(title) = title {
                ticket.title = title;
            }
            if let Some(description) = description {
                ticket.description = description;
            }
            if let Some(status) = status {
                ticket.status = Status::convert_string_to_status(&status)?;
            }
            if let Some(assignee) = assignee {
                ticket.assignee = Some(assignee);
            }
            ticket.updated_at = Utc::now();
            ticket_store.save(path)?;
            println!("Ticket updated successfully.");
            Ok(())
        }
    }
}