Theorem add_0 : forall n : nat, n + 0 = n.
Proof.
  intros n.
  simpl.
  reflexivity.
Qed.
