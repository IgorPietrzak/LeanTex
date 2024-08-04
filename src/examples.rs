pub const UNIV_MEM_PRINCIPAL: &'static str = r#"Statement {A : Set 𝓧} : univ ∈ 𝓟 A := by
  Hint "Start with `rw [mem_principal]`."
  rw [mem_principal]
  Hint "Now `apply` a theorem we proved already to finish the job. Take a look at the `Set` tab
  in the `Theorems` panel on the right to remind yourself of the theorems we've proved about sets."
  apply subset_univ
"#;

pub const LE_ANTISYMM: &'static str = r#"Statement le_antisymm {𝓕 𝓖 : Filter 𝓧} (h1 : 𝓕 ≤ 𝓖) (h2 : 𝓖 ≤ 𝓕) : 𝓕 = 𝓖 := by
  Hint "Two filters are equal if and only if they are the same collection of sets.
  This is an extensionality principle (two things are the same if they're made up of
  the same stuff). So start with `ext S`."
  ext S
  constructor
  Hint "Try using `rw[le_def] at *`"
  rw[le_def] at *
  Hint "Can you take it from here?"
  apply h2
  rw[le_def] at *
  apply h1
"#;

pub const INTER_COMM: &'static str = r#"Statement inter_comm (S T : Set 𝓧) : S ∩ T = T ∩ S := by
  Hint "Start with `ext a`."
  ext a
  Hint "We are trying to get this goal into a form where the `tauto` tactic
  will solve it. Click on the `tauto` tactic on the right to see what
  it does. Then try `rw [mem_inter_iff]`"
  rw [mem_inter_iff]
  Hint "Now do it again, and the goal will be purely a logic goal."
  rw [mem_inter_iff]
  Hint "This has now got nothing to do with sets. Prove this logic goal with `tauto`."
  tauto"#;

pub const COMAP_UNIV_MEM: &'static str = r#"Statement comap_univ_mem : univ ∈ 𝓖.comap φ := by
  sorry"#;

#[cfg(test)]
mod test {
    use crate::{examples, lean, regex_expr};
    use regex::Regex;
    #[test]
    fn examples() {
        let pattern = regex_expr::REGEX::new().pattern;
        let re = Regex::new(&pattern).unwrap();

        if let Some(caps) = re.captures(examples::INTER_COMM) {
            println!("\n\n INTER COMM \n ------- ");
            let lean = lean::LeanParsed::new(caps);
            println!("{:#?}", lean);
        } else {
            println!("No match found.");
        }
        if let Some(caps) = re.captures(examples::UNIV_MEM_PRINCIPAL) {
            println!("\n\n UNIV_MEM_PRINCIPAL \n ------- ");
            let lean = lean::LeanParsed::new(caps);
            println!("{:#?}", lean);
        } else {
            println!("No match found.");
        }
        if let Some(caps) = re.captures(examples::LE_ANTISYMM) {
            println!("\n\n LE_ANTISYMM \n ------- ");
            let lean = lean::LeanParsed::new(caps);
            println!("{:#?}", lean);
        } else {
            println!("No match found.");
        }
        if let Some(caps) = re.captures(examples::COMAP_UNIV_MEM) {
            println!("\n\n COMAP_UNIV_MEM \n ------- ");
            let lean = lean::LeanParsed::new(caps);
            println!("{:#?}", lean);
        } else {
            println!("No match found.");
        }
    }
}
