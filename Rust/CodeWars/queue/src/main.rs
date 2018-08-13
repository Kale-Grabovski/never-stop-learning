fn sjf(jobs: &[usize], index: usize) -> usize {
	jobs.iter()
		.enumerate()
		.filter(|&(i, &x)| x < jobs[index] || x == jobs[index] && i <= index)
		.map(|(_, x)| x)
		.sum()
}

#[test]
fn returns_expected() {
  assert_eq!(sjf(&[100], 0), 100);
  assert_eq!(sjf(&[3,10,20,1,2], 0), 6);
  assert_eq!(sjf(&[3,3,3,10,20,1,2], 1), 9);
}
