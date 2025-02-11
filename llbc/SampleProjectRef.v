(** THIS FILE WAS AUTOMATICALLY GENERATED BY AENEAS *)
(** [sample_project_ref] *)
Require Import Primitives.
Import Primitives.
Require Import Coq.ZArith.ZArith.
Require Import List.
Import ListNotations.
Local Open Scope Primitives_scope.
Module SampleProjectRef.

(** [sample_project_ref::fun_name]:
    Source: 'src/main.rs', lines 9:0-11:1 *)
Definition fun_name (x : i32) : result i32 :=
  i32_add x 1%i32.

(** [sample_project_ref::ref_incr]:
    Source: 'src/main.rs', lines 5:0-7:1 *)
Definition ref_incr (x : i32) : result i32 :=
  fun_name x.

(** [sample_project_ref::test_incr]:
    Source: 'src/main.rs', lines 13:0-17:1 *)
Definition test_incr : result unit :=
  y <- ref_incr 0%i32; massert (y s= 1%i32)
.

(** [sample_project_ref::main]:
    Source: 'src/main.rs', lines 1:0-3:1 *)
Definition main : result unit :=
  test_incr.

End SampleProjectRef.
