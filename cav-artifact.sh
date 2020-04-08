#!/usr/bin/env bash

RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

printf "${RED}arbiter-2-prompt${NC}\n\n"
printf "${CYAN}Build encoding with system bound 2 and exists bound 1 ${NC}\n"
./target/release/bosy --spec cav-samples/arbiter-2-prompt-1.bosy --smt cav-samples/arbiter-2-prompt-1.smt2 --bound 2 --bounds 1 > /dev/null 2> /dev/null
printf "${CYAN}Solve cav-samples/arbiter-2-prompt-1.smt2 using z3 (expected result unsat in < 1 sec)${NC}\n"
./external/bin/z3 cav-samples/arbiter-2-prompt-1.smt2
hyperfine "./external/bin/z3 cav-samples/arbiter-2-prompt-1.smt2"
printf "\n\n"
printf "${CYAN}Build encoding with system bound 2 and exists bound 2 ${NC}\n"
./target/release/bosy --spec cav-samples/arbiter-2-prompt-1.bosy --smt cav-samples/arbiter-2-prompt-2.smt2 --bound 2 --bounds 2 > /dev/null 2> /dev/null
printf "${CYAN}Solve cav-samples/arbiter-2-prompt-2.smt2 using z3 (expected result sat in < 1 sec)${NC}\n"
./external/bin/z3 cav-samples/arbiter-2-prompt-2.smt2
hyperfine "./external/bin/z3 cav-samples/arbiter-2-prompt-2.smt2"

printf "\n\n${RED}arbiter-2-full-prompt${NC}\n\n"
printf "${CYAN}Build encoding with system bound 3 and exists bound 1 ${NC}\n"
./target/release/bosy --spec cav-samples/arbiter-2-full-prompt-1.bosy --smt cav-samples/arbiter-2-full-prompt-1.smt2 --bound 3 --bounds 1 > /dev/null 2> /dev/null
printf "${CYAN}Solve cav-samples/arbiter-2-full-prompt-1.smt2 using z3 (expected result unsat in ~ 2 sec)${NC}\n"
./external/bin/z3 cav-samples/arbiter-2-full-prompt-1.smt2
hyperfine "./external/bin/z3 cav-samples/arbiter-2-full-prompt-1.smt2"
printf "\n\n"
printf "${CYAN}Build encoding with system bound 3 and exists bound 2 ${NC}\n"
./target/release/bosy --spec cav-samples/arbiter-2-full-prompt-1.bosy --smt cav-samples/arbiter-2-full-prompt-2.smt2 --bound 3 --bounds 2 > /dev/null 2> /dev/null
printf "${CYAN}Solve cav-samples/arbiter-2-full-prompt-2.smt2 using z3 (expected result sat in ~ 6 sec)${NC}\n"
./external/bin/z3 cav-samples/arbiter-2-full-prompt-2.smt2
hyperfine "./external/bin/z3 cav-samples/arbiter-2-full-prompt-2.smt2"

printf "\n\n${RED}arbiter-3-prompt${NC}\n\n"
printf "${CYAN}Build encoding with system bound 3 and exists bound 1 ${NC}\n"
./target/release/bosy --spec cav-samples/arbiter-3-prompt-1.bosy --smt cav-samples/arbiter-3-prompt-1.smt2 --bound 3 --bounds 1 > /dev/null 2> /dev/null
printf "${CYAN}Solve cav-samples/arbiter-3-prompt-1.smt2 using z3 (expected result unsat in ~ 5 sec)${NC}\n"
./external/bin/z3 cav-samples/arbiter-3-prompt-1.smt2
hyperfine "./external/bin/z3 cav-samples/arbiter-3-prompt-1.smt2"
printf "\n\n"
printf "${CYAN}Build encoding with system bound 3 and exists bound 2 ${NC}\n"
./target/release/bosy --spec cav-samples/arbiter-3-prompt-1.bosy --smt cav-samples/arbiter-3-prompt-2.smt2 --bound 3 --bounds 2 > /dev/null 2> /dev/null
printf "${CYAN}Solve cav-samples/arbiter-3-prompt-2.smt2 using z3 (expected result sat in ~ 8 sec)${NC}\n"
./external/bin/z3 cav-samples/arbiter-3-prompt-2.smt2
hyperfine "./external/bin/z3 cav-samples/arbiter-3-prompt-2.smt2"

printf "\n\n${RED}arbiter-4-prompt${NC}\n\n"
printf "${CYAN}Build encoding with system bound 4 and exists bound 1 ${NC}\n"
./target/release/bosy --spec cav-samples/arbiter-4-prompt-1.bosy --smt cav-samples/arbiter-4-prompt-1.smt2 --bound 4 --bounds 1 > /dev/null 2> /dev/null
printf "${CYAN}Solve cav-samples/arbiter-4-prompt-1.smt2 using z3 (expected result unsat in ~ 80 sec)${NC}\n"
./external/bin/z3 cav-samples/arbiter-4-prompt-1.smt2
hyperfine "./external/bin/z3 cav-samples/arbiter-4-prompt-1.smt2"

printf "\n\n${RED}dinning-cryptographers${NC}\n\n"
printf "${CYAN}Build encoding with system bound 1 and exists bound 1 ${NC}\n"
./target/release/bosy --spec cav-samples/dinning-cryptographers.bosy --smt cav-samples/dinning-cryptographers.smt2 --bound 1 --bounds 1,1,1,1,1,1,1 > /dev/null 2> /dev/null
printf "${CYAN}Solve cav-samples/dinning-cryptographers.smt2 using z3 (expected result sat in < 1 sec)${NC}\n"
./external/bin/z3 cav-samples/dinning-cryptographers.smt2
hyperfine "./external/bin/z3 cav-samples/dinning-cryptographers.smt2"

