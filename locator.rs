use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
#[derive(Debug)]
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {
        /* put item in beginning. */
        self.insert(0,ele);
        let mut num_nodes_in_tree:usize = self.len();
        /* println!("{}",num_nodes_in_tree); */

        /* create base cases for n less than 5, floatkeydown takes care of n >= 5 */
      /*   if num_nodes_in_tree == 1
        {
            ()
        }
        else if num_nodes_in_tree == 2
        {
            if self[1] < self[0]  
            {
                self.swap(0,1)  
            }    
        }
        else if num_nodes_in_tree == 3
        {
            if self[1] < self[0]  
            {
                self.swap(0,1)  
            }   
            if self[2] < self[0] 
            {
                self.swap(0,2)  
            }  
        }
         else if num_nodes_in_tree == 4
        {
            if self[1] < self[0]  
            {
                self.swap(0,1)  
            }   
            
            /* println!("{:?}",self[1]); */
             if self[3] < self[1]
            {
                self.swap(1,3)
            } 
            /* if self[1] < self[0]
            {
                self.swap(0,1)
            } */
        }  */


        
        
        
            /* println!("{}",num_nodes_in_tree);  */
            /* float it down to proper spot */
        float_key_down(self, 0, num_nodes_in_tree); 
        
    }

    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {
        if self.len() == 0
        {
            return None
        }
        else
        {
            let deleted = Some(self.remove(0));
           float_key_down(self, 0, self.len());
           return deleted;
          /*  float_key_down(self,2, self.len()-1); */
             
        }
    }

    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        if self.len() == 0
        {
            return None
        }
        else
        {
            return Some(&self[0]);
        }
    }
}




/* takes an element of vector tree i, and its length num and recursively floats
 it down to its proper place in relation to its children */
 fn float_key_down<T: PartialOrd>(tree: &mut Vec<T>, i:usize, num_nodes_in_tree:usize)->()
 {
    if num_nodes_in_tree == 1
    {
        ()
    }
    else if num_nodes_in_tree == 2
    {
        if tree[1] < tree[0]  
        {
            tree.swap(0,1)  
        }    
    }
    else if num_nodes_in_tree == 3
    {
        if tree[1] < tree[0]  
        {
            tree.swap(0,1)  
        }    
        if tree[2] < tree[0] 
        {
            tree.swap(0,2)  
        }  
    }
     else if num_nodes_in_tree == 4
    {
        if tree[1] < tree[0]  
        {
            tree.swap(0,1)  
        }   
        
        /* println!("{:?}",self[1]); */
         if tree[3] < tree[1]
        {
            tree.swap(1,3)
        } 
        /* if self[1] < self[0]
        {
            self.swap(0,1)
        } */
    } 

    else
    { 
        let mut leftnode:usize = 2* i + 1;
        let mut rightnode:usize = 2* i + 2;
        let mut smallestnode:usize = i;

        if leftnode <= num_nodes_in_tree && tree [ leftnode ] < tree [ smallestnode ]
        {
            smallestnode = leftnode;
        }

        if rightnode <= num_nodes_in_tree && tree [ rightnode ] < tree [ smallestnode ]
        {
            smallestnode = rightnode;
        }

        if smallestnode!= i
        {
        /*    let mut temp = &tree [i]; */
            tree.swap(i, smallestnode);
            float_key_down( tree , smallestnode , num_nodes_in_tree );
        }
   /*  } */
    }

 }
 /**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {

    let x_distance:i32 = p1.0 - p2.0;
    let y_distance:i32 = p1.1 - p2.1;
    
    let orthogonal_distance = x_distance.abs() + y_distance.abs();
    return orthogonal_distance;

}
 
/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/

 pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {
    /* permute every ally enemy possibility and calculate the distance of each permutation */
    let mut i:usize = 0;
    let mut q = Vec::new(); 
    let mut visited_enemies = Vec::new();

    for (enemy_name, enemy_loc) in enemies
    {
        visited_enemies.push(enemy_name);
    }

    for (ally_name, ally_loc) in allies
    {
        for (enemy_name, enemy_loc) in enemies
        {
            
            let dist = distance(*ally_loc, *enemy_loc);  
              
            q.enqueue(Node 
            {   
               
                priority: dist,
                /* we only want to own the villains name as the hero, starks name
                will be the one returned */
                data: enemy_name.to_owned().to_owned() + ally_name.to_owned()
                
            });


        }
    }
    let q_len:usize = q.len();
    let mut visited_enemies_len = visited_enemies.len();
    let mut j = 0;
    let mut prev_j = 0;
    let mut tup = ("", 0, 0); 

    while i < q_len
    {
        println!("i is {}",i);
        let node = q.dequeue().unwrap();
        
        while j < visited_enemies_len
        {
            println!("j is {}",j);
            println!("visited enemies list is currently {:?}",visited_enemies);
            println!("current node is {:?}", node);
            println!("checking if node contains {} ",visited_enemies[j]);
            if node.data.contains(visited_enemies[j].as_str()) && node.data.contains("Stark")
            {

                println!("Current node contains {} and Stark ",visited_enemies[j]);
                println!("found");
                let dist1 = enemies.get(visited_enemies[j]);
                let dist2 = enemies.get(visited_enemies[j]);
                tup = (visited_enemies[j].as_str(),dist1.unwrap().0,dist2.unwrap().1);
                /* prob wont need the below lines in final version */
             
                visited_enemies.remove(j);     
                visited_enemies_len = visited_enemies.len();   
                
            }
            
            else if node.data.contains(visited_enemies[j].as_str())
            {
                
                println!("removing {}",visited_enemies[j]);
                visited_enemies.remove(j);     
                visited_enemies_len = visited_enemies.len();    
                   
            }
            else
            {
                j += 1;
            }
             
        }
        j = 0;
  
        i += 1;
    }
    return tup;  
} 

/* fn main()
{
     let mut allies = HashMap::new();
    let stark = "Stark".to_string();
    let hulk = "Hulk".to_string();
    allies.insert(&stark, (6 as i32,3 as i32));
    allies.insert(&hulk, (1 as i32,4 as i32)); 

    let mut enemies = HashMap::new();
    let thanos = "Thanos".to_string();
    let ebony = "Ebony Maw".to_string();
   
    enemies.insert(&thanos, (4 as i32,1 as i32));
    enemies.insert(&ebony, (2 as i32,2 as i32)); 
    

    let mut i:usize = 0;
    let mut q = Vec::new(); 
    let mut visited_enemies = Vec::new();

    for (enemy_name, enemy_loc) in &enemies
    {
        visited_enemies.push(enemy_name);
    }
    println!("visited enmies list is currently {:?}",visited_enemies);

    for (ally_name, ally_loc) in &allies
    {
        for (enemy_name, enemy_loc) in &enemies
        {
     /*        println!("{:?}{:?} : {:?}{:?}", ally_name,enemy_name, ally_loc,enemy_loc); */
           
            /* for each ally enemy pairing, calculate the distance (priority) and
            store the ally-enemy name (data) */
            
            
            let dist = distance(*ally_loc, *enemy_loc);  
               /* println!("{}", dist); */
            q.enqueue(Node 
            {   
               
                priority: dist,
                /* we only want to own the villains name as the hero, starks name
                will be the one returned */
                data: enemy_name.to_owned().to_owned() + ally_name.to_owned()
                
            });
           /*  println!("{:?}",(Node 
            {   
               
                priority: dist,
                /* we only want to own the villains name as the hero, starks name
                will be the one returned */
                data: enemy_name.to_owned().to_owned() + ally_name.to_owned()
                
            })); */
           /*  println!("{}",q.len()); */


        }
    }
    let q_len:usize = q.len();
    let mut visited_enemies_len = visited_enemies.len();
    let mut j = 0;
    let mut prev_j = 0;
   /*  println!("current node is {:?}",q.peek()); */
   /*  println!("{:?}",q.dequeue().unwrap().data.contains("Stark"));  */
    while i < q_len
    {
        println!("i is {}",i);
        let node = q.dequeue().unwrap();
        
        while j < visited_enemies_len
        {
            println!("j is {}",j);
            println!("visited enemies list is currently {:?}",visited_enemies);
            println!("current node is {:?}", node);
            println!("checking if node contains {} ",visited_enemies[j]);
            if node.data.contains(visited_enemies[j].as_str()) && node.data.contains("Stark")
            {

                println!("Current node contains {} and Stark ",visited_enemies[j]);
                println!("found");
                /* prob wont need the below lines in final version */
                let dist1 = enemies.get(visited_enemies[j]);
                let dist2 = enemies.get(visited_enemies[j]);
                println!("Returned {:?}", (visited_enemies[j],dist1.unwrap().0,dist2.unwrap().1));
                visited_enemies.remove(j);     
                visited_enemies_len = visited_enemies.len();   
                
            }
            
            else if node.data.contains(visited_enemies[j].as_str())
            {
                
                println!("removing {}",visited_enemies[j]);
                visited_enemies.remove(j);     
                visited_enemies_len = visited_enemies.len();    
                
                
            }
            else
            {
                j += 1;
            }
            
            
        }
        j = 0;

    
        
        i += 1;
        /*  println!("{:?}",q.peek());  */

    }  
    
/*     let x:u32 = 6; 
    let dist = distance((12,22),(12,22)); */
    /* let node1 =Node{data: distance((12,22),(12,22)), priority:1};
    println!("{:?}",node1); */
    /* let mut q = Vec::new();  */
     /* q.enqueue(Node{data: distance((12,22),(12,22)), priority:1});  */
    /*  println!("{:?}",q); */

    /* assert_eq!(("Thanos", 4, 1), target_locator(&allies, &enemies));  */

   /*  println!("{}",distance((6, 3),(2,2)));  */
/* 
    let c1 = (5,5);
    let c2 = (3,2);

    println!("{}",distance(c1, c2)); */
  /*    let mut q = Vec::new();
     q.enqueue(3);
    q.enqueue(4);
    q.enqueue(5);
     q.enqueue(6); 
     q.enqueue(1);
 */
    

   /*  assert_eq!(1, q[0]);
    assert_eq!(4, q[1]); */
/*     println!("{:?}",q); 

    println!("dequeue: {:?}",q.dequeue());
    println!("{:?}",q); 
    println!("dequeue: {:?}",q.dequeue());
    println!("{:?}",q); 
    println!("dequeue: {:?}",q.dequeue());
    println!("{:?}",q); 
    println!("dequeue: {:?}",q.dequeue());
    println!("{:?}",q); 
    println!("dequeue: {:?}",q.dequeue());
    println!("{:?}",q); 
    println!("dequeue: {:?}",q.dequeue());
    println!("{:?}",q);  */

  /*   println!("peek: {:?}",q.peek());
    println!("{:?}",q); 
    println!("dequeue: {:?}",q.dequeue());
    println!("{:?}",q); 
    println!("peek: {:?}",q.peek());
    println!("{:?}",q); 
    println!("dequeue: {:?}",q.dequeue());
    println!("{:?}",q);  */
/*     println!("{:?}",assert_eq!(Some(&1), q.peek())); 
    println!("{:?}",assert_eq!(Some(1), q.dequeue()));  */
   // assert_eq!(Some(1), q.dequeue());
   // assert_eq!(Some(&3), q.peek());
   // assert_eq!(Some(3), q.dequeue());


   /*  let mut vec: Vec<i32> = Vec::new();
   /*  let mut vec = vec![]; */
     vec.insert(0, 4); 
    println!("{:?}",vec); */
}
 */
