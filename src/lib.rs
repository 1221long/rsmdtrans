

struct TransPart 
{
    start_positon: usize,
    end_position: usize,
    content: String,
    trans_success: bool,
    trans_content: String,
}

impl TransPart{
    fn new(sp: usize, ep: usize, c: String) -> Self {
        TransPart {
            start_positon: sp,
            end_position: ep,
            content: c,
            trans_success: false,
            trans_content: String::new(),
        }
    }
}



