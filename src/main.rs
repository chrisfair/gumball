    struct financial_picture{
       dollars_per_hour:i32,
       hours_per_week:i32,
       dollars_per_month:i32,
       rent_per_month:i32,
       food_per_month:i32,
       car_money_per_month:i32,
       electricity_per_month:i32,
       water_per_month:i32,
       business_income_per_month:i32,
       entertainment_per_month:i32,
    }


fn main() {

    let my_current_financial_picture = financial_picture; 
    
     
    financial_picure.dollars_per_hour = 10;

    let mut disposable_income = financial_picture.dollars_per_month +
                                business_income_per_month -
                                rent_per_month -
                                food_per_month -
                                car_money_per_month -
                                electricity_per_month -
                                water_per_month;

    let mut savings_per_month = disposable_income;

    println!("Gumball Business Simulator");
    println!("You have a job making {} dollars per hour", dollars_per_hour);
    println!("You work {} hours per week", hours_per_week);
    println!("Your rent is {} per month", rent_per_month);
    println!("Your food is {} per month", food_per_month);
    println!("Your electricity is {} per month", electricity_per_month);
    println!("Your water is {} per month", water_per_month);
    println!("Your disposable income is {} per month", disposable_income);
    println!("Right now my savings per month is {} dollars", savings_per_month);

    

}
