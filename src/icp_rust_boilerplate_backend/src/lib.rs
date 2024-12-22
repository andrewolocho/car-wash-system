#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// CarWashCenter struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CarWashCenter {
    id: u64,
    name: String,
    location: String,
    contact: String,
    email: String,
    capacity: u64, // Maximum number of vehicles handled per day
    created_at: u64,
}

// Vehicle struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Vehicle {
    id: u64,
    car_wash_id: u64,
    license_plate: String,
    vehicle_type: String, // e.g., car, truck, SUV
    arrival_time: u64,
    status: String,       // "received", "washing", "completed"
}

// Service struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Service {
    id: u64,
    car_wash_id: u64,
    vehicle_id: u64,
    service_type: String, // e.g., basic wash, premium wash, waxing
    price: f64,
    duration: u64,        // duration in minutes
    completed_at: Option<u64>,
}

// Expense struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Expense {
    id: u64,
    car_wash_id: u64,
    date: u64,
    category: String,
    amount: f64,
    description: String,
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateCarWashCenterPayload {
    name: String,
    location: String,
    contact: String,
    email: String,
    capacity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RegisterVehiclePayload {
    car_wash_id: u64,
    license_plate: String,
    vehicle_type: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateServicePayload {
    car_wash_id: u64,
    vehicle_id: u64,
    service_type: String,
    price: f64,
    duration: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordExpensePayload {
    car_wash_id: u64,
    category: String,
    amount: f64,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable for CarWashCenter
impl Storable for CarWashCenter {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for CarWashCenter {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Vehicle
impl Storable for Vehicle {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Vehicle {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Service
impl Storable for Service {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Service {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Expense
impl Storable for Expense {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Expense {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static CAR_WASH_CENTERS: RefCell<StableBTreeMap<u64, CarWashCenter, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    static VEHICLES: RefCell<StableBTreeMap<u64, Vehicle, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));

    static SERVICES: RefCell<StableBTreeMap<u64, Service, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    static EXPENSES: RefCell<StableBTreeMap<u64, Expense, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));
}

// Functions

// Create Car Wash Center
#[ic_cdk::update]
fn create_car_wash_center(payload: CreateCarWashCenterPayload) -> Result<CarWashCenter, Message> {
    if payload.name.is_empty() || payload.contact.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let car_wash_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let car_wash = CarWashCenter {
        id: car_wash_id,
        name: payload.name,
        location: payload.location,
        contact: payload.contact,
        email: payload.email,
        capacity: payload.capacity,
        created_at: time(),
    };

    CAR_WASH_CENTERS.with(|centers| {
        centers.borrow_mut().insert(car_wash_id, car_wash.clone());
    });

    Ok(car_wash)
}

// Register Vehicle
#[ic_cdk::update]
fn register_vehicle(payload: RegisterVehiclePayload) -> Result<Vehicle, Message> {
    if payload.license_plate.is_empty() || payload.vehicle_type.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let car_wash_exists = CAR_WASH_CENTERS.with(|centers| centers.borrow().contains_key(&payload.car_wash_id));
    if !car_wash_exists {
        return Err(Message::NotFound("Car Wash Center not found".to_string()));
    }

    let vehicle_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let vehicle = Vehicle {
        id: vehicle_id,
        car_wash_id: payload.car_wash_id,
        license_plate: payload.license_plate,
        vehicle_type: payload.vehicle_type,
        arrival_time: time(),
        status: "received".to_string(),
    };

    VEHICLES.with(|vehicles| {
        vehicles.borrow_mut().insert(vehicle_id, vehicle.clone());
    });

    Ok(vehicle)
}

// Create Service
#[ic_cdk::update]
fn create_service(payload: CreateServicePayload) -> Result<Service, Message> {
    if payload.service_type.is_empty() || payload.price <= 0.0 || payload.duration <= 0 {
        return Err(Message::InvalidPayload("Invalid service data".to_string()));
    }

    let car_wash_exists = CAR_WASH_CENTERS.with(|centers| centers.borrow().contains_key(&payload.car_wash_id));
    if !car_wash_exists {
        return Err(Message::NotFound("Car Wash Center not found".to_string()));
    }

    let vehicle_exists = VEHICLES.with(|vehicles| vehicles.borrow().contains_key(&payload.vehicle_id));
    if !vehicle_exists {
        return Err(Message::NotFound("Vehicle not found".to_string()));
    }

    let service_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let service = Service {
        id: service_id,
        car_wash_id: payload.car_wash_id,
        vehicle_id: payload.vehicle_id,
        service_type: payload.service_type,
        price: payload.price,
        duration: payload.duration,
        completed_at: None,
    };

    SERVICES.with(|services| {
        services.borrow_mut().insert(service_id, service.clone());
    });

    Ok(service)
}

// Record Expense
#[ic_cdk::update]
fn record_expense(payload: RecordExpensePayload) -> Result<Expense, Message> {
    if payload.amount <= 0.0 {
        return Err(Message::InvalidPayload("Invalid expense amount".to_string()));
    }

    let car_wash_exists = CAR_WASH_CENTERS.with(|centers| centers.borrow().contains_key(&payload.car_wash_id));
    if !car_wash_exists {
        return Err(Message::NotFound("Car Wash Center not found".to_string()));
    }

    let expense_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let expense = Expense {
        id: expense_id,
        car_wash_id: payload.car_wash_id,
        date: time(),
        category: payload.category,
        amount: payload.amount,
        description: payload.description,
    };

    EXPENSES.with(|expenses| {
        expenses.borrow_mut().insert(expense_id, expense.clone());
    });

    Ok(expense)
}

// Calculate Total Revenue
#[ic_cdk::query]
fn calculate_total_revenue(car_wash_id: u64) -> Result<f64, Message> {
    let car_wash_exists = CAR_WASH_CENTERS.with(|centers| centers.borrow().contains_key(&car_wash_id));
    if !car_wash_exists {
        return Err(Message::NotFound("Car Wash Center not found".to_string()));
    }

    let total_revenue: f64 = SERVICES.with(|services| {
        services
            .borrow()
            .iter()
            .filter(|(_, service)| service.car_wash_id == car_wash_id)
            .map(|(_, service)| service.price)
            .sum()
    });

    Ok(total_revenue)
}

// Calculate Total Expenses
#[ic_cdk::query]
fn calculate_total_expenses(car_wash_id: u64) -> Result<f64, Message> {
    let car_wash_exists = CAR_WASH_CENTERS.with(|centers| centers.borrow().contains_key(&car_wash_id));
    if !car_wash_exists {
        return Err(Message::NotFound("Car Wash Center not found".to_string()));
    }

    let total_expenses: f64 = EXPENSES.with(|expenses| {
        expenses
            .borrow()
            .iter()
            .filter(|(_, expense)| expense.car_wash_id == car_wash_id)
            .map(|(_, expense)| expense.amount)
            .sum()
    });

    Ok(total_expenses)
}

// Exporting the candid interface
ic_cdk::export_candid!();
