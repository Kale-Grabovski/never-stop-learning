fn main() {
	// A &str (pronounced "stir" or "string slice") is a reference to a run of UTF-8 text 
    // owned by someone else: it “borrows” the text. In the example, oodles is a &str refer‐ 
	// ring to the last six bytes of the text belonging to noodles, so it represents the text 
	// "oodles". Like other slice references, a &str is a fat pointer, containing both the 
	// address of the actual data and its length. You can think of a &str as being nothing 
	// more than a &[u8] that is guaranteed to hold well-formed UTF-8.
	assert_eq!("޵_޵".len(), 5); 
	assert_eq!("޵_޵".chars().count(), 3);

	// The type &mut str does exist, but it is not very useful, since almost any operation on 
	// UTF-8 can change its overall byte length, and a slice cannot reallocate its referent. In 
	// fact, the only operations available on &mut str are make_ascii_uppercase and 
	// make_ascii_lowercase, which modify the text in place and affect only single-byte 
	// characters, by definition.
	let bits = vec!["veni", "vidi", "vici"]; 
	assert_eq!(bits.concat(), "venividivici"); 
	assert_eq!(bits.join(", "), "veni, vidi, vici");
	assert_eq!(format!("{}°{:02}′{:02}″N", 24, 5, 23), "24°05′23″N".to_string());

	assert!("peanut".contains("nut"));
	assert_eq!(" clean\n".trim(), "clean"); 
	for word in "veni, vidi, vici".split(", ") { 
		assert!(word.starts_with("v")); 
	}
}
