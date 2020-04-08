(set-option :smt.ematching false)
(set-option :smt.mbqi true)
(declare-datatype S ( (s_0) (s_1) (s_2) ))
(declare-datatype Q ( (T0_init) (accept_S2) (accept_S3) (accept_S4) ))
(declare-datatype Q_0 ( (q_0_T0_init) (q_0_accept_S1) (q_0_accept_S2) (q_0_accept_S3) (q_0_accept_S4) (q_0_accept_S5) (q_0_accept_S6) ))
(declare-datatype S_0_0 ( (s_0_0_0) ))
(declare-fun tau (S Bool Bool Bool Bool) S)
(declare-fun grant1 (S Bool Bool Bool Bool) Bool)
(declare-fun grant2 (S Bool Bool Bool Bool) Bool)
(declare-fun grant3 (S Bool Bool Bool Bool) Bool)
(declare-fun lambda_ (S Q) Bool)
(declare-fun lambda_sharp (S Q) Int)
(declare-fun tau_0_0 (S_0_0) S_0_0)
(declare-fun out_0_0_request1_pi_ (S_0_0) Bool)
(declare-fun out_0_0_request2_pi_ (S_0_0) Bool)
(declare-fun out_0_0_request3_pi_ (S_0_0) Bool)
(declare-fun out_0_0_color_pi_ (S_0_0) Bool)
(declare-fun lambda_0 (S S Q_0 S_0_0) Bool)
(declare-fun lambda_0_sharp (S S Q_0 S_0_0) Int)
(assert (lambda_ s_0 T0_init ))
(assert (forall ((s S) (s_p S) (request1 Bool) (request2 Bool) (request3 Bool) (color Bool)) (=> (and (and (and (lambda_ s T0_init ) (or (and (and (not (grant1 s request1 request2 request3 color ) ) (not (grant2 s request1 request2 request3 color ) ) ) request1 ) (and (and (not (grant1 s request1 request2 request3 color ) ) (not (grant3 s request1 request2 request3 color ) ) ) request1 ) ) ) (and true (= (tau s request1 request2 request3 color ) s_p ) ) ) true ) (and (lambda_ s_p accept_S2 ) (< (lambda_sharp s T0_init ) (lambda_sharp s_p accept_S2 ) ) ) )))
(assert (forall ((s S) (s_p S) (request1 Bool) (request2 Bool) (request3 Bool) (color Bool)) (=> (and (and (and (lambda_ s T0_init ) (or (or (and (not (grant1 s request1 request2 request3 color ) ) (not (grant2 s request1 request2 request3 color ) ) ) (and (not (grant1 s request1 request2 request3 color ) ) (not (grant3 s request1 request2 request3 color ) ) ) ) (and (not (grant2 s request1 request2 request3 color ) ) (not (grant3 s request1 request2 request3 color ) ) ) ) ) (and true (= (tau s request1 request2 request3 color ) s_p ) ) ) true ) (and (lambda_ s_p T0_init ) (<= (lambda_sharp s T0_init ) (lambda_sharp s_p T0_init ) ) ) )))
(assert (forall ((s S) (s_p S) (request1 Bool) (request2 Bool) (request3 Bool) (color Bool)) (=> (and (and (and (lambda_ s T0_init ) (or (and (and (not (grant1 s request1 request2 request3 color ) ) (not (grant2 s request1 request2 request3 color ) ) ) request2 ) (and (and (not (grant2 s request1 request2 request3 color ) ) (not (grant3 s request1 request2 request3 color ) ) ) request2 ) ) ) (and true (= (tau s request1 request2 request3 color ) s_p ) ) ) true ) (and (lambda_ s_p accept_S3 ) (< (lambda_sharp s T0_init ) (lambda_sharp s_p accept_S3 ) ) ) )))
(assert (forall ((s S) (s_p S) (request1 Bool) (request2 Bool) (request3 Bool) (color Bool)) (=> (and (and (and (lambda_ s T0_init ) (or (and (and (not (grant1 s request1 request2 request3 color ) ) (not (grant3 s request1 request2 request3 color ) ) ) request3 ) (and (and (not (grant2 s request1 request2 request3 color ) ) (not (grant3 s request1 request2 request3 color ) ) ) request3 ) ) ) (and true (= (tau s request1 request2 request3 color ) s_p ) ) ) true ) (and (lambda_ s_p accept_S4 ) (< (lambda_sharp s T0_init ) (lambda_sharp s_p accept_S4 ) ) ) )))
(assert (forall ((s S) (s_p S) (request1 Bool) (request2 Bool) (request3 Bool) (color Bool)) (=> (and (and (and (lambda_ s accept_S2 ) (not (grant1 s request1 request2 request3 color ) ) ) (and true (= (tau s request1 request2 request3 color ) s_p ) ) ) true ) (and (lambda_ s_p accept_S2 ) (< (lambda_sharp s accept_S2 ) (lambda_sharp s_p accept_S2 ) ) ) )))
(assert (forall ((s S) (s_p S) (request1 Bool) (request2 Bool) (request3 Bool) (color Bool)) (=> (and (and (and (lambda_ s accept_S3 ) (not (grant2 s request1 request2 request3 color ) ) ) (and true (= (tau s request1 request2 request3 color ) s_p ) ) ) true ) (and (lambda_ s_p accept_S3 ) (< (lambda_sharp s accept_S3 ) (lambda_sharp s_p accept_S3 ) ) ) )))
(assert (forall ((s S) (s_p S) (request1 Bool) (request2 Bool) (request3 Bool) (color Bool)) (=> (and (and (and (lambda_ s accept_S4 ) (not (grant3 s request1 request2 request3 color ) ) ) (and true (= (tau s request1 request2 request3 color ) s_p ) ) ) true ) (and (lambda_ s_p accept_S4 ) (< (lambda_sharp s accept_S4 ) (lambda_sharp s_p accept_S4 ) ) ) )))
(assert (lambda_0 s_0 s_0 q_0_T0_init s_0_0_0 ))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_T0_init s_exists_0 ) (and (and (not (out_0_0_color_pi_ s_exists_0 ) ) (not (grant1 s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) ) ) request1_pip ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S3 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_T0_init s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S3 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_T0_init s_exists_0 ) (and (and (not (out_0_0_color_pi_ s_exists_0 ) ) (not (grant1 s_pip request1_pip request2_pip request3_pip color_pip ) ) ) request1_pip ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S6 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_T0_init s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S6 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_T0_init s_exists_0 ) true ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_T0_init s_exists_p_0 ) (<= (lambda_0_sharp s_pi s_pip q_0_T0_init s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_T0_init s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_T0_init s_exists_0 ) (and (and (out_0_0_color_pi_ s_exists_0 ) (not (grant1 s_pip request1_pip request2_pip request3_pip color_pip ) ) ) request1_pip ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S5 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_T0_init s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S5 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_T0_init s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S2 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_T0_init s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S2 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_T0_init s_exists_0 ) (and (and (out_0_0_color_pi_ s_exists_0 ) (not (grant1 s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) ) ) request1_pip ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S4 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_T0_init s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S4 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_T0_init s_exists_0 ) (not (out_0_0_color_pi_ s_exists_0 ) ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S1 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_T0_init s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S1 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_accept_S1 s_exists_0 ) (not (out_0_0_color_pi_ s_exists_0 ) ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S1 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_accept_S1 s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S1 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_accept_S2 s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S2 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_accept_S2 s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S2 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_accept_S3 s_exists_0 ) (and (not (out_0_0_color_pi_ s_exists_0 ) ) (not (grant1 s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) ) ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S3 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_accept_S3 s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S3 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_accept_S4 s_exists_0 ) (and (not (out_0_0_color_pi_ s_exists_0 ) ) (not (grant1 s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) ) ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S3 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_accept_S4 s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S3 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_accept_S4 s_exists_0 ) (and (out_0_0_color_pi_ s_exists_0 ) (not (grant1 s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) ) ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S4 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_accept_S4 s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S4 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_accept_S5 s_exists_0 ) (and (out_0_0_color_pi_ s_exists_0 ) (not (grant1 s_pip request1_pip request2_pip request3_pip color_pip ) ) ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S5 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_accept_S5 s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S5 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_accept_S6 s_exists_0 ) (and (not (out_0_0_color_pi_ s_exists_0 ) ) (not (grant1 s_pip request1_pip request2_pip request3_pip color_pip ) ) ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S6 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_accept_S6 s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S6 s_exists_p_0 ) ) ) )))
(assert (forall ((s_pi S) (s_pip S) (s_p_pi S) (s_p_pip S) (s_exists_0 S_0_0) (s_exists_p_0 S_0_0) (request1_pip Bool) (request2_pip Bool) (request3_pip Bool) (color_pip Bool)) (=> (and (and (and (lambda_0 s_pi s_pip q_0_accept_S6 s_exists_0 ) (and (out_0_0_color_pi_ s_exists_0 ) (not (grant1 s_pip request1_pip request2_pip request3_pip color_pip ) ) ) ) (and (and true (= (tau s_pi (out_0_0_request1_pi_ s_exists_0 ) (out_0_0_request2_pi_ s_exists_0 ) (out_0_0_request3_pi_ s_exists_0 ) (out_0_0_color_pi_ s_exists_0 ) ) s_p_pi ) ) (= (tau s_pip request1_pip request2_pip request3_pip color_pip ) s_p_pip ) ) ) (and true (= (tau_0_0 s_exists_0 ) s_exists_p_0 ) ) ) (and (lambda_0 s_p_pi s_p_pip q_0_accept_S5 s_exists_p_0 ) (< (lambda_0_sharp s_pi s_pip q_0_accept_S6 s_exists_0 ) (lambda_0_sharp s_p_pi s_p_pip q_0_accept_S5 s_exists_p_0 ) ) ) )))

(check-sat)

