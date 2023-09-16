//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region: --- Ticket Types
#[derive(Debug, Serialize, Clone)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64, // creator user_id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}
// endregion: --- Ticket Types

// region: --- Model Controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        /// 创建票据
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        // 创建了一个store变量，它是一个互斥锁，用于保护门票信息的存储。在这里使用了lock()方法获取互斥锁，并使用unwrap()方法将其转换为Result类型。
        let store = self.tickets_store.lock().unwrap();
        // 使用了store.iter()方法遍历门票信息，并使用filter_map()方法过滤掉无效的门票信息。最后使用collect()方法将有效的门票信息收集到一个Vec中。
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        // 这段代码的作用是删除一个票据。它首先获取了一个可变的引用store，然后使用id作为索引从store中获取票据并将其移除。如果成功获取到票据，则返回该票据；否则返回一个包含错误信息的Result。
        let mut store = self.tickets_store.lock().unwrap();
        // 使用id作为索引从store中获取票据，并使用and_then方法对获取的票据进行操作。在这里，使用了一个闭包(|t| t.take())来获取票据并将其从store中移除。
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

// endregion: --- Model Controller
