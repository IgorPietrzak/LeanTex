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
