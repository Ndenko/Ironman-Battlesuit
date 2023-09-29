#[derive(Debug)]
#[derive(PartialEq)]

pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {
        match self
        {
            Command::Power(true,n) => String::from("Power increased by ".to_owned()+&n.to_string()+"%"),
            Command::Power(false,n) => String::from("Power decreased by ".to_owned()+&n.to_string()+"%"),
            Command::Missiles(true,n) => String::from("Missiles increased by ".to_owned()+&n.to_string()),
            Command::Missiles(false,n) => String::from("Missiles decreased by ".to_owned()+&n.to_string()),
            Command::Shield(true) => String::from("Shield turned on"),
            Command::Shield(false) => String::from("Shield turned off"),
            Command::Try => String::from("Call attempt failed"),
            Command::Invalid => String::from("Not a command"),                               
            _ => String::from("Not a command")
        }  
    }
}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/

pub fn to_command(s: &str) -> Command {
    let mut valid = true;
    if &s[0..5] == "power"
    {
        if &s[5..10] == " inc "
        {
            let mut i = 10;
            /* make sure everything that follows is a number */
            while i < s.len()
            {
                /* println!("{} {}",i,valid); */
                if &s[i..i+1] != "0" &&  &s[i..i+1] != "1" && &s[i..i+1]  != "2" && &s[i..i+1]  != "3" && &s[i..i+1]  != "4"
                && &s[i..i+1]  != "5" && &s[i..i+1]  != "6" && &s[i..i+1]  != "7" && &s[i..i+1]  != "8" && &s[i..i+1]  != "9"
                {
                    valid = false;
                }
                i += 1
               
            }
            if valid 
            {
                let my_int = (&s[10..]).parse::<i32>().unwrap();
                return Command::Power(true, my_int); 
            }
            else 
            {    
                return Command::Invalid;  
            }
            
                      
            
        }
        else if &s[5..10] == " dec "
        {
            let mut i = 10;
            /* make sure everything that follows is a number */
            while i < s.len()
            {
                /* println!("{} {}",i,valid); */
                if &s[i..i+1] != "0" &&  &s[i..i+1] != "1" && &s[i..i+1]  != "2" && &s[i..i+1]  != "3" && &s[i..i+1]  != "4"
                && &s[i..i+1]  != "5" && &s[i..i+1]  != "6" && &s[i..i+1]  != "7" && &s[i..i+1]  != "8" && &s[i..i+1]  != "9"
                {
                    valid = false;
                }
                i += 1
               
            }
            if valid 
            {
                let my_int = (&s[10..]).parse::<i32>().unwrap();
                return Command::Power(false, my_int); 
            }
            else 
            {    
                return Command::Invalid;  
            }
        }
        else 
        {
            return Command::Invalid;   
        }
        
    }
    /* Missiles    |  /(fire|add) [0-9]+ missiles/ */
    else if &s[0..5] == "fire "
    {
        let mut i = 5;
        let mut j = 0;

        while i < s.len() && j == 0
        {
            println!("i:{}, {}",i,&s[i..i+1]);
            if &s[i..i+1] != "0" &&  &s[i..i+1] != "1" && &s[i..i+1]  != "2" && &s[i..i+1]  != "3" && &s[i..i+1]  != "4"
            && &s[i..i+1]  != "5" && &s[i..i+1]  != "6" && &s[i..i+1]  != "7" && &s[i..i+1]  != "8" && &s[i..i+1]  != "9"
            {
                j = i;
            }
            i += 1
           
        }
        /* if j > s.len() - 9, fail */

        if &s[j..s.len()] == " missiles"
        {
            println!("j:{}, {}",j,&s[j..j+1]);
            let my_int = (&s[5..j]).parse::<i32>().unwrap();
            return Command::Missiles(false, my_int);
        }
        else 
        {
            return Command::Invalid;   
        }

    }

    else if &s[0..4] == "add "
    {
        let mut i = 4;
        let mut j = 0;

        while i < s.len() && j == 0
        {
            println!("i:{}, {}",i,&s[i..i+1]);
            if &s[i..i+1] != "0" &&  &s[i..i+1] != "1" && &s[i..i+1]  != "2" && &s[i..i+1]  != "3" && &s[i..i+1]  != "4"
            && &s[i..i+1]  != "5" && &s[i..i+1]  != "6" && &s[i..i+1]  != "7" && &s[i..i+1]  != "8" && &s[i..i+1]  != "9"
            {
                j = i;
            }
            i += 1
           
        }
        /* if j > s.len() - 9, fail */

        if &s[j..s.len()] == " missiles"
        {
            println!("j:{}, {}",j,&s[j..j+1]);
            let my_int = (&s[4..j]).parse::<i32>().unwrap();
            return Command::Missiles(true, my_int);
        }
        else 
        {
            return Command::Invalid;   
        }
    }
    else if &s[0..s.len()] == "shield on"
    {
        return Command::Shield(true);
    }
    else if &s[0..s.len()] == "shield off"
    {
        return Command::Shield(false);
    }
    else if &s[0..3] == "try"
    {
        return Command::Try;
    }

    else 
    {
        return Command::Invalid;

    }
    
}
pub fn str_to_int(s: &str) -> i32 {
    match s
    {
        "" => 1,
        _ => 1
    }

}

fn main()
{
   
    /* assert_eq!(Command::Power(true,60), to_command("power inc 60"));
    assert_eq!(Command::Power(false,30), to_command("power dec 30")); */
/*     println!("{:?}",to_command("power inc 333"));
    println!("{:?}",to_command("power dec 193")); */
   /*  println!("{:?}",to_command("fire 227 missiles"));
    println!("{:?}",to_command("add 7 missiles")); */

    assert_eq!("Call attempt failed", Command::Try.as_str());
    assert_eq!("Not a command", Command::Invalid.as_str());

    assert_eq!(Command::Try, to_command("try calling Miss Potts"));
    assert_eq!(Command::Invalid, to_command("jarvis!"));
    assert_eq!("Shield turned on", Command::Shield(true).as_str());
    assert_eq!("Shield turned off", Command::Shield(false).as_str());

    assert_eq!(Command::Shield(true), to_command("shield on"));
    assert_eq!(Command::Shield(false), to_command("shield off"));
    assert_eq!("Missiles increased by 60", Command::Missiles(true, 60).as_str());
    assert_eq!("Missiles decreased by 30", Command::Missiles(false, 30).as_str());

    assert_eq!(Command::Missiles(true,60), to_command("add 60 missiles"));
    assert_eq!(Command::Missiles(false,30), to_command("fire 30 missiles"));
    assert_eq!("Power increased by 60%", Command::Power(true, 60).as_str());
    assert_eq!("Power decreased by 30%", Command::Power(false, 30).as_str());

    assert_eq!(Command::Power(true,60), to_command("power inc 60"));
    assert_eq!(Command::Power(false,30), to_command("power dec 30"));

}