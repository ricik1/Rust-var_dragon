/*
I32 (signed 32-bit integer)done
f64 (floating point number)done
bool (boolean)done
char (character)done
string (string)done

*/

//import library
// library for input and output
use std::io;
use colored::*;
//added use colored to add color to every line
//creaet entery point to our program the main fn
fn main(){

//I tabbed over to keep the writing on the page starting at the same spot
// print the title of the porgram
let msg: ColoredString = "\n\n\t\t\t**** Welcome to Rusty the Dragon's Sanctuary of Healing ****\n\n".red();
println!("{msg}");
//added colorstring use colored to add color to every line
//a short into to the sanctuary
let msg: ColoredString = "\n\n\t\t\t   Welcome to the Sanctuary of Healing, a place where magic and 
                compassion intertwine to mend both body and soul. Here, ancient remedies meet modern wonders,
                and the warmth of care envelops all who seek solace and restoration. Step into a realm of hope
                and rejuvenation, where the skilled healers and kind-hearted beings work together to heal the
                wounds of the world. Whether you arrive in search of physical well-being or inner peace,
                know that within these sacred walls, you will find respite and renewal. Welcome to the
                Sanctuary of Healing, where miracles are nurtured and where every heart is welcomed with open arms.".red().bold();
println!("{}",msg);

//assignement requirement string
//create a var to hold the users first name
// remeber mut is used to make the var mutable which means changeable

//I tabbed over to keep the writing on the page starting at the same spot
let mut first_name = String::new();
//ask for name
let msg: ColoredString ="\n\n\t\t    Oh majestic and illustrious dragon, might
                I be blessed with knowledge of the name that echoes 
                through the annals of time? What title, imbued with the
                essence of your grandeur, do you bear as you soar through 
                the ages, leaving your mark upon the tapestry of ancient legends?\n\n".bright_green();
println!("{msg}");


//get the users input and store it in the first_name var
io::stdin().read_line(&mut first_name).expect("failed to read pt first name");
// trim or remove any extra spaces before and after the user input
//we do this becasue it make the compare easrier later on
first_name = first_name.trim().to_string();


//Here i am welcoming our pt by name
let msg: ColoredString = format!( "\t\t We are here to heal, tending to dragons with compassion and ancient wisdom, {}", first_name).yellow().bold().italic();//by adding a space after the coma it will add the space before the anme
println!("{}",msg);

//assignmeent requirement char var
//ask the user for the pt gender
// create a var to hold the pt last name
//create a var to hold the pt gender
let mut gender= String::new();
//''means a char="" means a string
//I tabbed over to keep the writing on the page starting at the same spot

let msg: ColoredString ="\n\n \t\t  Oh majestic dragon, whose aura glimmers like
                a constellation in the night sky, I tread upon a delicate
                question with utmost reverence. Might you, noble being, 
                indulge my curiosity and share the essence of your being?
                Does your majestic form embody the spirit of a gallant
                dragoness or the valor of a gallant dragon? (M/F/O)\n\n".bright_green();
println!("{msg}");

//get pt gender and store it in string var gender
io::stdin().read_line( &mut gender).expect("failed tp get patients gender");
//lets convert the string to a char
//the char is what we will use to hold the data
// the string was for input

let c_gender = gender.trim().chars().next().unwrap();

//thanking pt for answering the question
let msg: ColoredString = format!( "\t\t We extend our heartfelt gratitude to the majestic, {} for sharing your gender with us!", first_name).yellow().bold().italic();//by adding a space after the coma it will add the space before the anme
println!("{}",msg);

//I tabbed over to keep the writing on the page starting at the same spot
//assignmeent requirement i32- bit interger var
//ask the user for the pt age
//create a string var to hold the pt age
//create a var to hold the pt age
let mut age = String::new();
// ask the user for the age
let msg: ColoredString ="\n\n\t\t   Oh mighty dragon, guardian of ancient
                wisdom and the embodiment of timeless strength, 
                may I dare to inquire about the wondrous years 
                that grace your existence? How many cycles of
                seasons have you traversed since your fiery birth,
                as you soared among the stars and witnessed the 
                unfolding of ages past?\n\n".bright_green();
println!("{msg}");

io::stdin().read_line(&mut age).expect("failed to read patient's age");
//now lets covert the string to an interger
//we do this becasue it is easy to validate the users input with a string
//let age: i32 = age.trim().parse().expect("Please type in a number!");
let age: i32 = age.trim().parse().expect("Please type in a number!");

//thanking pt for answering the question
let msg: ColoredString = format!( "\t\t  {} In the realm of dragons, age carries the weight of timeless wisdom and ancient tales.", first_name).yellow().bold().italic();//by adding a space after the coma it will add the space before the anme
println!("{}",msg);


//assignment requirement (f64 (floating point number)) var
// creat a string to get the pts temp
//input the is a sting which is the best choice for input
let mut temperature = String::new();

//I tabbed over to keep the writing on the page starting at the same spot
//ask do for pt temp
let msg: ColoredString ="\n\n\t\t   Oh mighty and venerable dragon, whose fiery breath ignites
                the heavens and stirs the very essence of wonder, might you grace
                this humble soul with knowledge of the temperature at which your 
                inferno blazes? Is your fire a celestial furnace that burns with 
                the brilliance of a thousand suns, or does it hold secrets that 
                leave even the most daring scholars in awe??\n\n".bright_green();
println!("{msg}");

io::stdin().read_line(&mut temperature).expect("Failed ot read patient's temperature");
let temp: f64 = temperature.trim().parse().expect("Please type a number!");

//thanking pt for answering the question
let msg: ColoredString = format!( "\t\t  {}, We extend our sincerest gratitude to the magnificent dragon
                for sharing its awe-inspiring and powerful temperature with us.", first_name).yellow().bold().italic();//by adding a space after the coma it will add the space before the anme
println!("{}",msg);

//assignment requirement (f64 (floating point number)) var
// creat a string to get the pts height
//input the is a sting which is the best choice for input
let mut height = String::new();

//I tabbed over to keep the writing on the page starting at the same spot
//ask do for pt temp
let msg: ColoredString ="\n\n\t\t   Oh mighty dragon, may I humbly inquire about your impressive height,
                     which surely reaches the heavens and beyond?\n\n".bright_green();
println!("{msg}");

io::stdin().read_line(&mut height).expect("Failed ot read patient's temperature");
let height: f64 = height.trim().parse().expect("Please type a number!");

//thanking pt for answering the question
let msg: ColoredString = format!( "\t\t  {}, Great dragon, we express our deepest appreciation
                 for your towering height, a sight that fills our hearts with wonder and admiration.", first_name).yellow().bold().italic();//by adding a space after the coma it will add the space before the anme
println!("{}",msg);





//Assignment required (bool (boolean)) var
//still uise a string for input
let mut insurance = String::new();
//I tabbed over to keep the writing on the page starting at the same spot
//ask the doc if the pt has insurance
let msg: ColoredString ="\n\n\t\t   Oh magnificent dragon, whose wisdom spans across
                eons and whose aura resonates with timeless grace, forgive me
                for the audacity of this query. Yet, in the world of humans,
                it is said that some protect themselves from unforeseen perils
                with the safeguard of insurance. In this realm of magical and
                mortal intermingling, does the concept of insurance find a
                place in your domain? Does the dragon, with all its might 
                and mystique, too partake in the affairs of insurance, or
                does the essence of your being transcend such earthly concerns? (Y/N)\n\n".bright_green();
println!("{msg}");

//get insurance and stuff the value inot the string var insurance
io::stdin().read_line(&mut insurance).expect("fail to read patients's insurance");

//letting pt kmnow that the doc will be right in
let msg: ColoredString = format!( "\t\t  {} Fear not, noble dragon, for the esteemed healer
                shall be with you shortly, attending to your needs with utmost care and reverence.", first_name).yellow().bold().italic();//by adding a space after the coma it will add the space before the anme
println!("{}",msg);

//Now lets convert the string into a boonlean based on the users input.
//if they tryped Y or y then the value is true
// if they typed N or n then the value is false
let is_Insured: bool = match insurance.trim().to_lowercase().as_ref(){
    "y"=> true,
    "n"=> false,
    _=> panic!("Please type Y or N"),
    //ideas could add cost of visit without insuracne math problem with cost per day
    //add pulse, heart rate, height, wing spand, print out temp if too cold or too hot if/else
    
};

                //does your pt have insurance we collected on pt
                let msg: ColoredString =format!("\n\n<<< Patient Information >>>").yellow();
                println!("{}",msg);

                let msg: ColoredString =format!("--------------------").yellow();
                println!("{}",msg);

                // print out the pt first name
                let msg: ColoredString =format!("Dragon's name:{}", first_name).yellow();
                println!("{}",msg);

                // print out the pt gender
                let msg: ColoredString =format!(" Dragon's gender:{}", c_gender).yellow();
                println!("{}",msg);
                // print out the pt age
                let msg: ColoredString =format!(" Dragon's age:{}", age).yellow();
                println!("{}",msg);
                // print out the pt temp
                let msg: ColoredString =format!(" Dragon's temperature:{}", temp).yellow();
                println!("{}",msg);
                // print out the pt height
                let msg: ColoredString =format!(" Dragon's height:{}", height).yellow();
                println!("{}",msg);
                // print out the pt first ins
                let msg: ColoredString =format!(" Dragon's insured:{}", is_Insured).yellow();
                println!("{}",msg);
                //print out a goodbye message
                let msg: ColoredString =format!("\n\n\t\t\tThank you for using Rusty the Dragon's Sanctuary of Healing Simulator\n\n").red();
                println!("{}",msg);



}

