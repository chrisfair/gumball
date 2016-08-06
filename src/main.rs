use std::io;

struct financial_picture_description{
    dollars_per_hour:f32,
    hours_per_week:f32,
    dollars_per_month:f32,
    rent_per_month:f32,
    food_per_month:f32,
    car_money_per_month:f32,
    electricity_per_month:f32,
    water_per_month:f32,
    business_income_per_month:f32,
    business_expenses_per_month:f32,
    entertainment_per_month:f32,
    disposable_income:f32,
    savings_per_month:f32,
    current_savings:f32
}

struct gumball_machine_description{

    number_of_gumballs: i32,
    cost_per_gumball: f32,
    price_per_gumball: f32,
    number_of_gumballs_sold: i32,
    price_of_gumball_machine: f32,
    performance_of_gumball_machine: i32

}

fn initialized_financial_picture () -> financial_picture_description{

    let mut financial_picture = financial_picture_description { 
        dollars_per_hour:10.0,
        hours_per_week:40.0,
        dollars_per_month:0.0,
        rent_per_month:800.0,
        food_per_month:300.0,
        car_money_per_month:200.0,
        electricity_per_month:100.0,
        water_per_month:100.0,
        business_income_per_month:0.0,
        business_expenses_per_month:0.0,
        entertainment_per_month:0.0,
        disposable_income:0.0,
        savings_per_month:0.0,
        current_savings:0.0};

    financial_picture.dollars_per_month = financial_picture.dollars_per_hour *
        4.0 * financial_picture.hours_per_week;

    financial_picture.disposable_income = financial_picture.dollars_per_month +
        financial_picture.business_income_per_month - 
        financial_picture.rent_per_month -
        financial_picture.food_per_month -
        financial_picture.car_money_per_month -
        financial_picture.water_per_month -
        financial_picture.entertainment_per_month;

    financial_picture.savings_per_month = financial_picture.disposable_income - 
                                          financial_picture.entertainment_per_month;

    financial_picture
}

fn print_initial_description(financial_picture: &mut financial_picture_description)
{
    println!("Gumball Business Simulator");
    println!("You have a job making {} dollars per hour", financial_picture.dollars_per_hour);
    println!("You work {} hours per week", financial_picture.hours_per_week);
    println!("Your make {} dollars per month", financial_picture.dollars_per_month);
    println!("Your rent is {} per month", financial_picture.rent_per_month);
    println!("Your food is {} per month", financial_picture.food_per_month);
    println!("Your electricity is {} per month", financial_picture.electricity_per_month);
    println!("Your water is {} per month", financial_picture.water_per_month);
}

fn print_current_state(financial_picture: &mut financial_picture_description)
{

    println!("Your disposable income is ${}", financial_picture.disposable_income);
    println!("Right now your savings is ${}", financial_picture.current_savings);

}


fn main() {

    let mut financial_picture = initialized_financial_picture();
    let mut current_month = 0;
    let mut current_year = 0;
    let mut current_age = 18;
    let starting_age:i32 = current_age; 
    let mut number_of_gumball_machines:i32 = 0;
    let mut price_of_gumball_machine:f32 = 80.0;


    print_initial_description(&mut financial_picture);

    loop
    {
        let mut entertainment_to_spend: f32 = financial_picture.disposable_income + 
                                              financial_picture.current_savings + 1;
        let max_spend = entertainment_to_spend - 1;
        
        let mut gumball_machines_to_buy: i32 = financial_picture.disposable_income / price_per_gumball +
                                               financial_picture.current_savings + 1;
        let max_gumballs = gumball_machines_to_buy - 1;
        current_age = starting_age + current_year;

        print_current_state(&mut financial_picture);

        println!("You are currently {} years old", current_age);
        println!("You have been running your gumball business for {} months",current_month);

       
        while entertainment_to_spend > max_spend
        {

            entertainment_to_spend = how_much_to_spend_on_entertainment();

        }

        while gumball_machines_to_buy > max_gumballs
        {

            gumball_machines_to_buy = how_many_gumball_machines_to_buy(); 
        }

        current_month = current_month + 1;
        current_year = (current_month / 12);


    }
}


fn how_much_to_spend_on_entertainment()->f32
{
        println!("How much do you want to spend on entertainment this month?");

        let mut entertainment = String::new();
        
        io::stdin().read_line(&mut entertainment)
            .expect("Failed to read line");

        let entertainment: f32 = entertainment.trim().parse()
            .expect("Please enter dollars to spend on entertainment");
        entertainment
}


fn how_many_gumball_machines_to_buy()->i32
{

        println!("How many gumball machines do you want to buy this month?");
        let mut gumball_machines = String::new();
        
        io::stdin().read_line(&mut gumball_machines)
            .expect("Failed to read line");

        let gumball_machines: i32 = gumball_machines.trim().parse()
            .expect("Please enter the number of gumball machines to buy");

        gumball_machines
        



}

