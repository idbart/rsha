
// pad the source data
pub fn pad(bytes: &mut Vec<u8>)
{
    // get the length of the message in bits
    let l: u64 = (bytes.len() * 8) as u64;
    // calcualte the number of 0s needed in padding
    let mut k = {
        // the amount of required bits of padding 
        let other_padding  = 65;

        // ill find a way to explain this later
        if bytes.len() % 64 <= 55
        {
            ((64 - (bytes.len() % 64)) * 8) - other_padding
        }
        else 
        {
            (((64 - (bytes.len() % 64)) * 8) + 512) - other_padding
        }
    }; 

    // finally, add the padding
    // starting with the 1 at the start 
    bytes.push(0b10000000);
    k -= 7;

    // then add all the 0s needed
    for i in 0..(k / 8)
    {
        bytes.push(0b00000000);
    }

    // then add the 64 bit number containing the length of the message
    for i in l.to_ne_bytes().iter()
    {
        bytes.push(*i);
    }
}