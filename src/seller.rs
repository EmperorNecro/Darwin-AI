
#[derive(Copy, Clone)]
pub struct Seller {
    pub minsellingpoint:i32,
    profit: i32,
    char: char,
    color: i32
}
//"Base functions " for the seller struct. Not class, as I wish to avoid OOP programming. 
impl Seller {
     // New seller class function
    pub fn new(minsellingpoint: i32, profit: i32, char: char, color:i32) -> Self {
        Seller { minsellingpoint, profit, char, color}
       
    }
    //Adjust the min selling point, called by other classes, just references self as needs to adjust function son sell. 
    pub fn change(&mut self,sell:bool){
         
      if sell
      {
        self.profit +=1;
      }
      else
      { 
        let comparable = self.profit - 1;
        if comparable == self.minsellingpoint
        {

          return;
        }
        self.profit -=1;
        
      }

      
      
    }




    
}

