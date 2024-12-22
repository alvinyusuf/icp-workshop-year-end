// #[macro_use]
// extern crate serde;
// use candid::{Decode, Encode};
// use ic_cdk::api::time;
// use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
// use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
// use std::{borrow::Cow, cell::RefCell};

// type Memory = VirtualMemory<DefaultMemoryImpl>;
// type IdCell = Cell<u64, Memory>;

// #[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
// struct Message {
//     id: u64,
//     title: String,
//     body: String,
//     attachment_url: String,
//     created_at: u64,
//     updated_at: Option<u64>,
// }

// // a trait that must be implemented for a struct that is stored in a stable struct
// impl Storable for Message {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }
// }

// // another trait that must be implemented for a struct that is stored in a stable struct
// impl BoundedStorable for Message {
//     const MAX_SIZE: u32 = 1024;
//     const IS_FIXED_SIZE: bool = false;
// }

// thread_local! {
//     static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
//         MemoryManager::init(DefaultMemoryImpl::default())
//     );

//     static ID_COUNTER: RefCell<IdCell> = RefCell::new(
//         IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
//             .expect("Cannot create a counter")
//     );

//     static STORAGE: RefCell<StableBTreeMap<u64, Message, Memory>> =
//         RefCell::new(StableBTreeMap::init(
//             MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
//     ));
// }

// #[derive(candid::CandidType, Serialize, Deserialize, Default)]
// struct MessagePayload {
//     title: String,
//     body: String,
//     attachment_url: String,
// }

// #[ic_cdk::query]
// fn get_message(id: u64) -> Result<Message, Error> {
//     match _get_message(&id) {
//         Some(message) => Ok(message),
//         None => Err(Error::NotFound {
//             msg: format!("a message with id={} not found", id),
//         }),
//     }
// }

// #[ic_cdk::update]
// fn add_message(message: MessagePayload) -> Option<Message> {
//     let id = ID_COUNTER
//         .with(|counter| {
//             let current_value = *counter.borrow().get();
//             counter.borrow_mut().set(current_value + 1)
//         })
//         .expect("cannot increment id counter");
//     let message = Message {
//         id,
//         title: message.title,
//         body: message.body,
//         attachment_url: message.attachment_url,
//         created_at: time(),
//         updated_at: None,
//     };
//     do_insert(&message);
//     Some(message)
// }

// #[ic_cdk::update]
// fn update_message(id: u64, payload: MessagePayload) -> Result<Message, Error> {
//     match STORAGE.with(|service| service.borrow().get(&id)) {
//         Some(mut message) => {
//             message.attachment_url = payload.attachment_url;
//             message.body = payload.body;
//             message.title = payload.title;
//             message.updated_at = Some(time());
//             do_insert(&message);
//             Ok(message)
//         }
//         None => Err(Error::NotFound {
//             msg: format!(
//                 "couldn't update a message with id={}. message not found",
//                 id
//             ),
//         }),
//     }
// }

// // helper method to perform insert.
// fn do_insert(message: &Message) {
//     STORAGE.with(|service| service.borrow_mut().insert(message.id, message.clone()));
// }

// #[ic_cdk::update]
// fn delete_message(id: u64) -> Result<Message, Error> {
//     match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
//         Some(message) => Ok(message),
//         None => Err(Error::NotFound {
//             msg: format!(
//                 "couldn't delete a message with id={}. message not found.",
//                 id
//             ),
//         }),
//     }
// }

// #[derive(candid::CandidType, Deserialize, Serialize)]
// enum Error {
//     NotFound { msg: String },
// }

// // a helper method to get a message by id. used in get_message/update_message
// fn _get_message(id: &u64) -> Option<Message> {
//     STORAGE.with(|service| service.borrow().get(id))
// }

// // need this to generate candid
// ic_cdk::export_candid!();

#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Copy, PartialEq)]
enum TransactionType {
    Income,
    Expense,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Copy, PartialEq)]
enum Category {
    Salary,
    Food,
    Transportation,
    Entertainment,
    Utilities,
    Others,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Transaction {
    id: u64,
    amount: f64,
    transaction_type: TransactionType,
    category: Category,
    description: String,
    date: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Budget {
    id: u64,
    category: Category,
    limit: f64,
    spent: f64,
    month: u8,
    year: u32,
}

impl Storable for Transaction {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Transaction {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Budget {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Budget {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static TRANSACTION_STORAGE: RefCell<StableBTreeMap<u64, Transaction, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static BUDGET_STORAGE: RefCell<StableBTreeMap<u64, Budget, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize)]
struct TransactionPayload {
    amount: f64,
    transaction_type: TransactionType,
    category: Category,
    description: String,
}

#[derive(candid::CandidType, Serialize, Deserialize)]
struct BudgetPayload {
    category: Category,
    limit: f64,
    month: u8,
    year: u32,
}

#[derive(candid::CandidType, Serialize, Deserialize)]
struct Summary {
    total_income: f64,
    total_expense: f64,
    balance: f64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

// Transaction Functions
#[ic_cdk::update]
fn add_transaction(payload: TransactionPayload) -> Result<Transaction, Error> {
    if payload.amount <= 0.0 {
        return Err(Error::InvalidInput {
            msg: "Amount must be greater than 0".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let transaction = Transaction {
        id,
        amount: payload.amount,
        transaction_type: payload.transaction_type,
        category: payload.category,
        description: payload.description,
        date: time(),
    };

    TRANSACTION_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, transaction.clone())
    });

    Ok(transaction)
}

#[ic_cdk::query]
fn get_transaction(id: u64) -> Result<Transaction, Error> {
    match TRANSACTION_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(transaction) => Ok(transaction),
        None => Err(Error::NotFound {
            msg: format!("Transaction with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_transactions_by_category(category: Category) -> Vec<Transaction> {
    TRANSACTION_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|(_, transaction)| transaction.category == category)
            .map(|(_, transaction)| transaction)
            .collect()
    })
}

#[ic_cdk::query]
fn get_balance() -> f64 {
    TRANSACTION_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .fold(0.0, |acc, (_, transaction)| {
                match transaction.transaction_type {
                    TransactionType::Income => acc + transaction.amount,
                    TransactionType::Expense => acc - transaction.amount,
                }
            })
    })
}

// Budget Functions
#[ic_cdk::update]
fn set_budget(payload: BudgetPayload) -> Result<Budget, Error> {
    if payload.limit <= 0.0 {
        return Err(Error::InvalidInput {
            msg: "Budget limit must be greater than 0".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let budget = Budget {
        id,
        category: payload.category,
        limit: payload.limit,
        spent: 0.0,
        month: payload.month,
        year: payload.year,
    };

    BUDGET_STORAGE
        .with(|storage| storage.borrow_mut().insert(id, budget.clone()));

    Ok(budget)
}

#[ic_cdk::query]
fn get_remaining_budget(category: Category, month: u8, year: u32) -> f64 {
    let total_spent = TRANSACTION_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|(_, transaction)| {
                transaction.transaction_type == TransactionType::Expense 
                && transaction.category == category
            })
            .map(|(_, transaction)| transaction.amount)
            .sum::<f64>()
    });

    BUDGET_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, budget)| {
                budget.category == category
                    && budget.month == month
                    && budget.year == year
            })
            .map(|(_, budget)| budget.limit - total_spent)
            .unwrap_or(0.0)
    })
}

// Summary Functions
#[ic_cdk::query]
fn get_monthly_summary(month: u8, year: u32) -> Summary {
    let (total_income, total_expense) = TRANSACTION_STORAGE.with(|storage| {
        let mut income = 0.0;
        let mut expense = 0.0;

        // Convert timestamp to month and year
        for (_, transaction) in storage.borrow().iter() {
            let timestamp = transaction.date;
            let transaction_date = time_to_date(timestamp);
            
            if transaction_date.month == month && transaction_date.year == year {
                match transaction.transaction_type {
                    TransactionType::Income => income += transaction.amount,
                    TransactionType::Expense => expense += transaction.amount,
                }
            }
        }
        (income, expense)
    });

    Summary {
        total_income,
        total_expense,
        balance: total_income - total_expense,
    }
}

// Helper struct dan fungsi untuk konversi timestamp
struct Date {
    year: u32,
    month: u8,
}

fn time_to_date(timestamp: u64) -> Date {
    // Convert nanoseconds to seconds
    let seconds = (timestamp / 1_000_000_000) as i64;
    
    // Basic conversion (this is simplified)
    let year = 1970 + (seconds / 31536000) as u32;
    let month = ((seconds % 31536000) / 2592000) as u8 + 1;
    
    Date { year, month }
}

// need this to generate candid
ic_cdk::export_candid!();