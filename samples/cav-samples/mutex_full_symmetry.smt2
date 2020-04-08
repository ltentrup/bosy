(declare-datatype S ( (s_0) (s_1) (s_2) ))
(declare-datatype Q ( (T0_init) (accept_all) (accept_S2) (accept_S3) (T0_S4) (T0_S5) (T0_S6) ))
(declare-datatype Q_0 ( (q_0_T0_init) (q_0_accept_all) ))
(declare-datatype S_0_1 ( (s_0_1_0) ))
(declare-fun tau (S Bool Bool Bool) S)
(declare-fun g1 (S Bool Bool Bool) Bool)
(declare-fun g2 (S Bool Bool Bool) Bool)
(declare-fun lambda_ (S Q) Bool)
(declare-fun lambda_sharp (S Q) Int)
(declare-fun tau_0_1 (S_0_1 Bool Bool Bool) S_0_1)
(declare-fun out_0_1_r1_pi2_ (S_0_1 Bool Bool Bool) Bool)
(declare-fun out_0_1_r2_pi2_ (S_0_1 Bool Bool Bool) Bool)
(declare-fun out_0_1_tie_pi2_ (S_0_1 Bool Bool Bool) Bool)
(declare-fun lambda_0 (S S Q_0 S_0_1) Bool)
(declare-fun lambda_0_sharp (S S Q_0 S_0_1) Int)
(assert (lambda_ s_0 T0_init )
	)
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_init )
	 (or (and (and (not (g2 s r1 r2 tie )
	 )
	 r1 )
	 (not r2 )
	 )
	 (and (and (not (g1 s r1 r2 tie )
	 )
	 (not (g2 s r1 r2 tie )
	 )
	 )
	 (not r2 )
	 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p T0_S5 )
	 (<= (lambda_sharp s T0_init )
	 (lambda_sharp s_p T0_S5 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_init )
	 (or (and (and (not (g1 s r1 r2 tie )
	 )
	 (not r1 )
	 )
	 r2 )
	 (and (and (not (g1 s r1 r2 tie )
	 )
	 (not (g2 s r1 r2 tie )
	 )
	 )
	 (not r1 )
	 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p T0_S4 )
	 (<= (lambda_sharp s T0_init )
	 (lambda_sharp s_p T0_S4 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_init )
	 (or (and (and (not (g1 s r1 r2 tie )
	 )
	 r1 )
	 r2 )
	 (and (and (not (g1 s r1 r2 tie )
	 )
	 (not (g2 s r1 r2 tie )
	 )
	 )
	 r1 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_S2 )
	 (< (lambda_sharp s T0_init )
	 (lambda_sharp s_p accept_S2 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_init )
	 (or (and (and (not (g2 s r1 r2 tie )
	 )
	 r1 )
	 r2 )
	 (and (and (not (g1 s r1 r2 tie )
	 )
	 (not (g2 s r1 r2 tie )
	 )
	 )
	 r2 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_S3 )
	 (< (lambda_sharp s T0_init )
	 (lambda_sharp s_p accept_S3 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_init )
	 (or (or (and (not (g2 s r1 r2 tie )
	 )
	 r1 )
	 (and (not (g1 s r1 r2 tie )
	 )
	 r2 )
	 )
	 (and (not (g1 s r1 r2 tie )
	 )
	 (not (g2 s r1 r2 tie )
	 )
	 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p T0_S6 )
	 (<= (lambda_sharp s T0_init )
	 (lambda_sharp s_p T0_S6 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_init )
	 (or (or (and (g1 s r1 r2 tie )
	 (g2 s r1 r2 tie )
	 )
	 (and (g1 s r1 r2 tie )
	 (not r1 )
	 )
	 )
	 (and (g2 s r1 r2 tie )
	 (not r2 )
	 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_all )
	 (< (lambda_sharp s T0_init )
	 (lambda_sharp s_p accept_all )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s accept_all )
	 true )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_all )
	 (< (lambda_sharp s accept_all )
	 (lambda_sharp s_p accept_all )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s accept_S2 )
	 (not (g1 s r1 r2 tie )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_S2 )
	 (< (lambda_sharp s accept_S2 )
	 (lambda_sharp s_p accept_S2 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s accept_S3 )
	 (not (g2 s r1 r2 tie )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_S3 )
	 (< (lambda_sharp s accept_S3 )
	 (lambda_sharp s_p accept_S3 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_S4 )
	 (and (g1 s r1 r2 tie )
	 (not r1 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_all )
	 (< (lambda_sharp s T0_S4 )
	 (lambda_sharp s_p accept_all )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_S4 )
	 (and (not (g1 s r1 r2 tie )
	 )
	 (not r1 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p T0_S4 )
	 (<= (lambda_sharp s T0_S4 )
	 (lambda_sharp s_p T0_S4 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_S5 )
	 (and (not (g2 s r1 r2 tie )
	 )
	 (not r2 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p T0_S5 )
	 (<= (lambda_sharp s T0_S5 )
	 (lambda_sharp s_p T0_S5 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_S5 )
	 (and (g2 s r1 r2 tie )
	 (not r2 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_all )
	 (< (lambda_sharp s T0_S5 )
	 (lambda_sharp s_p accept_all )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_S6 )
	 (and (g1 s r1 r2 tie )
	 (g2 s r1 r2 tie )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_all )
	 (< (lambda_sharp s T0_S6 )
	 (lambda_sharp s_p accept_all )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_S6 )
	 (and (not (g1 s r1 r2 tie )
	 )
	 r1 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_S2 )
	 (< (lambda_sharp s T0_S6 )
	 (lambda_sharp s_p accept_S2 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_S6 )
	 (or (not (g1 s r1 r2 tie )
	 )
	 (not (g2 s r1 r2 tie )
	 )
	 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p T0_S6 )
	 (<= (lambda_sharp s T0_S6 )
	 (lambda_sharp s_p T0_S6 )
	 )
	 )
	 )
	))
(assert (forall ((s S) (s_p S) (r1 Bool) (r2 Bool) (tie Bool)) (=> (and (and (and (lambda_ s T0_S6 )
	 (and (not (g2 s r1 r2 tie )
	 )
	 r2 )
	 )
	 (and true (= (tau s r1 r2 tie )
	 s_p )
	 )
	 )
	 true )
	 (and (lambda_ s_p accept_S3 )
	 (< (lambda_sharp s T0_S6 )
	 (lambda_sharp s_p accept_S3 )
	 )
	 )
	 )
	))
(assert (lambda_0 s_0 s_0 q_0_T0_init s_0_1_0 )
	)
(assert (forall ((s_pi1 S) (s_pi2 S) (s_p_pi1 S) (s_p_pi2 S) (s_exists_0 S_0_1) (s_exists_p_0 S_0_1) (r1_pi1 Bool) (r2_pi1 Bool) (tie_pi1 Bool)) (=> (and (and (and (lambda_0 s_pi1 s_pi2 q_0_T0_init s_exists_0 )
	 (or (or (or (or (or (or (or (and (not (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 (not (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 )
	 (and (not (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (not (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (not r1_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and r1_pi1 (not (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (not (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 r2_pi1 )
	 )
	 (and (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (not r2_pi1 )
	 )
	 )
	 )
	 (and (and true (= (tau s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 s_p_pi1 )
	 )
	 (= (tau s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 s_p_pi2 )
	 )
	 )
	 (and true (= (tau_0_1 s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 s_exists_p_0 )
	 )
	 )
	 (and (lambda_0 s_p_pi1 s_p_pi2 q_0_accept_all s_exists_p_0 )
	 (< (lambda_0_sharp s_pi1 s_pi2 q_0_T0_init s_exists_0 )
	 (lambda_0_sharp s_p_pi1 s_p_pi2 q_0_accept_all s_exists_p_0 )
	 )
	 )
	 )
	))
(assert (forall ((s_pi1 S) (s_pi2 S) (s_p_pi1 S) (s_p_pi2 S) (s_exists_0 S_0_1) (s_exists_p_0 S_0_1) (r1_pi1 Bool) (r2_pi1 Bool) (tie_pi1 Bool)) (=> (and (and (and (lambda_0 s_pi1 s_pi2 q_0_T0_init s_exists_0 )
	 (or (or (or (or (or (or (or (or (or (or (or (or (or (or (or (and (and (and (and (and (and (and (not (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (not (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not r1_pi1 )
	 )
	 (not (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r2_pi1 )
	 )
	 (not (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (and (and (and (and (and (and (not (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (not (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not r1_pi1 )
	 )
	 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 r2_pi1 )
	 (not (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (and (and (and (and (and (and (not (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (not (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 r1_pi1 )
	 (not (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r2_pi1 )
	 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (and (and (and (and (and (and (not (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (not (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 r1_pi1 )
	 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 r2_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (and (and (and (and (and (and (not (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (not (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not r1_pi1 )
	 )
	 (not (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r2_pi1 )
	 )
	 (not (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (and (and (and (and (and (and (not (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (not (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not r1_pi1 )
	 )
	 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 r2_pi1 )
	 (not (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (and (and (and (and (and (and (not (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (not (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 r1_pi1 )
	 (not (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r2_pi1 )
	 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (and (and (and (and (and (and (not (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (not (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 r1_pi1 )
	 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 r2_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (and (and (and (and (and (and (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 (not (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r1_pi1 )
	 )
	 (not (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r2_pi1 )
	 )
	 (not (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (and (and (and (and (and (and (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 (not (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r1_pi1 )
	 )
	 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 r2_pi1 )
	 (not (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (and (and (and (and (and (and (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 (not (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 r1_pi1 )
	 (not (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r2_pi1 )
	 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (and (and (and (and (and (and (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 (not (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (not (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 r1_pi1 )
	 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 r2_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (and (and (and (and (and (and (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r1_pi1 )
	 )
	 (not (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r2_pi1 )
	 )
	 (not (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (and (and (and (and (and (and (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r1_pi1 )
	 )
	 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 r2_pi1 )
	 (not (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (and (and (and (and (and (and (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 r1_pi1 )
	 (not (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (not r2_pi1 )
	 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (and (and (and (and (and (and (and (g1 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 (g1 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 (g2 s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 (g2 s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 r1_pi1 )
	 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 r2_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 )
	 )
	 (and (and true (= (tau s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 s_p_pi1 )
	 )
	 (= (tau s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 s_p_pi2 )
	 )
	 )
	 (and true (= (tau_0_1 s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 s_exists_p_0 )
	 )
	 )
	 (and (lambda_0 s_p_pi1 s_p_pi2 q_0_T0_init s_exists_p_0 )
	 (<= (lambda_0_sharp s_pi1 s_pi2 q_0_T0_init s_exists_0 )
	 (lambda_0_sharp s_p_pi1 s_p_pi2 q_0_T0_init s_exists_p_0 )
	 )
	 )
	 )
	))
(assert (forall ((s_pi1 S) (s_pi2 S) (s_p_pi1 S) (s_p_pi2 S) (s_exists_0 S_0_1) (s_exists_p_0 S_0_1) (r1_pi1 Bool) (r2_pi1 Bool) (tie_pi1 Bool)) (=> (and (and (and (lambda_0 s_pi1 s_pi2 q_0_accept_all s_exists_0 )
	 true )
	 (and (and true (= (tau s_pi1 r1_pi1 r2_pi1 tie_pi1 )
	 s_p_pi1 )
	 )
	 (= (tau s_pi2 (out_0_1_r1_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_r2_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 (out_0_1_tie_pi2_ s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 )
	 s_p_pi2 )
	 )
	 )
	 (and true (= (tau_0_1 s_exists_0 r1_pi1 r2_pi1 tie_pi1 )
	 s_exists_p_0 )
	 )
	 )
	 (and (lambda_0 s_p_pi1 s_p_pi2 q_0_accept_all s_exists_p_0 )
	 (< (lambda_0_sharp s_pi1 s_pi2 q_0_accept_all s_exists_0 )
	 (lambda_0_sharp s_p_pi1 s_p_pi2 q_0_accept_all s_exists_p_0 )
	 )
	 )
	 )
	))

(check-sat)

