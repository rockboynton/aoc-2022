import sys, re
import pdb
file = """Valve DR has flow rate=22; tunnels lead to valves DC, YA
Valve IO has flow rate=14; tunnels lead to valves GE, CK, HY, XB
Valve XY has flow rate=0; tunnels lead to valves IP, AR
Valve UQ has flow rate=0; tunnels lead to valves XU, PD
Valve FO has flow rate=0; tunnels lead to valves DL, NC
Valve PU has flow rate=0; tunnels lead to valves ZJ, AN
Valve MK has flow rate=0; tunnels lead to valves ZS, SB
Valve HN has flow rate=0; tunnels lead to valves AA, DV
Valve XF has flow rate=0; tunnels lead to valves XB, AA
Valve OD has flow rate=13; tunnels lead to valves ZS, AF, SY, QQ, AR
Valve GE has flow rate=0; tunnels lead to valves KR, IO
Valve UF has flow rate=18; tunnels lead to valves QQ, AN, YE, GY
Valve WK has flow rate=19; tunnel leads to valve PQ
Valve PQ has flow rate=0; tunnels lead to valves WK, CW
Valve XU has flow rate=0; tunnels lead to valves DV, UQ
Valve SH has flow rate=0; tunnels lead to valves IP, AA
Valve SY has flow rate=0; tunnels lead to valves ZJ, OD
Valve OU has flow rate=0; tunnels lead to valves CK, DL
Valve IP has flow rate=8; tunnels lead to valves CY, ML, YI, XY, SH
Valve XZ has flow rate=0; tunnels lead to valves AM, PD
Valve ZU has flow rate=0; tunnels lead to valves CW, SB
Valve DC has flow rate=0; tunnels lead to valves CF, DR
Valve QY has flow rate=0; tunnels lead to valves CW, MQ
Valve XB has flow rate=0; tunnels lead to valves IO, XF
Valve AF has flow rate=0; tunnels lead to valves PD, OD
Valve GY has flow rate=0; tunnels lead to valves UF, ZC
Valve ZC has flow rate=0; tunnels lead to valves GY, CW
Valve ZJ has flow rate=25; tunnels lead to valves SY, PU
Valve NC has flow rate=6; tunnels lead to valves HY, ML, NJ, AT, FO
Valve DS has flow rate=0; tunnels lead to valves AT, DV
Valve DV has flow rate=7; tunnels lead to valves FD, KR, HN, DS, XU
Valve HY has flow rate=0; tunnels lead to valves NC, IO
Valve WF has flow rate=0; tunnels lead to valves NJ, AA
Valve CK has flow rate=0; tunnels lead to valves IO, OU
Valve YE has flow rate=0; tunnels lead to valves CY, UF
Valve LA has flow rate=0; tunnels lead to valves DL, ZM
Valve QQ has flow rate=0; tunnels lead to valves OD, UF
Valve AM has flow rate=0; tunnels lead to valves XZ, SB
Valve AN has flow rate=0; tunnels lead to valves UF, PU
Valve CL has flow rate=16; tunnels lead to valves YA, LD
Valve CF has flow rate=12; tunnel leads to valve DC
Valve FD has flow rate=0; tunnels lead to valves DV, DL
Valve QU has flow rate=0; tunnels lead to valves LD, PD
Valve AT has flow rate=0; tunnels lead to valves DS, NC
Valve SB has flow rate=24; tunnels lead to valves MK, AM, ZU
Valve YI has flow rate=0; tunnels lead to valves DL, IP
Valve ZM has flow rate=0; tunnels lead to valves AA, LA
Valve LD has flow rate=0; tunnels lead to valves CL, QU
Valve AR has flow rate=0; tunnels lead to valves OD, XY
Valve DL has flow rate=5; tunnels lead to valves FO, LA, YI, OU, FD
Valve MQ has flow rate=0; tunnels lead to valves QY, PD
Valve PD has flow rate=9; tunnels lead to valves MQ, QU, XZ, AF, UQ
Valve KR has flow rate=0; tunnels lead to valves GE, DV
Valve CY has flow rate=0; tunnels lead to valves YE, IP
Valve AA has flow rate=0; tunnels lead to valves SH, XF, ZM, HN, WF
Valve NJ has flow rate=0; tunnels lead to valves NC, WF
Valve YA has flow rate=0; tunnels lead to valves CL, DR
Valve ML has flow rate=0; tunnels lead to valves NC, IP
Valve CW has flow rate=15; tunnels lead to valves QY, PQ, ZC, ZU
Valve ZS has flow rate=0; tunnels lead to valves MK, OD
"""
lines = [re.split('[\\s=;,]+', x) for x in file.splitlines()]

ADJACENT_MAP = {x[1]: set(x[10:]) for x in lines}
FLOW_MAP = {x[1]: int(x[5]) for x in lines if int(x[5]) != 0}
BITMASK_MAP = {x: 1<<i for i, x in enumerate(FLOW_MAP)}
# EDGES_MAP = {x: {y: 1 if y in ADJACENT_MAP[x] else float('+inf') for y in ADJACENT_MAP} for x in ADJACENT_MAP}
EDGES_MAP = {}
for valve_from in ADJACENT_MAP:
    edge_exists = {}
    for valve_to in ADJACENT_MAP:
        edge_exists[valve_to] = 1 if valve_to in ADJACENT_MAP[valve_from] else float('+inf')
    EDGES_MAP[valve_from] = edge_exists

# pdb.set_trace()
for k in EDGES_MAP:
    for i in EDGES_MAP:
        for j in EDGES_MAP:
            EDGES_MAP[i][j] = min(EDGES_MAP[i][j], EDGES_MAP[i][k]+EDGES_MAP[k][j])

def visit(current_valve, budget, state, flow, answer):
    answer[state] = max(answer.get(state, 0), flow)
    for next_valve in FLOW_MAP:
        newbudget = budget - EDGES_MAP[current_valve][next_valve] - 1
        if BITMASK_MAP[next_valve] & state or newbudget <= 0:
            continue
        else:
            visit(next_valve, newbudget, state | BITMASK_MAP[next_valve], flow + newbudget * FLOW_MAP[next_valve], answer)
    return answer

import time
start1 = time.perf_counter()
total1 = max(visit('AA', 30, 0, 0, {}).values())
end1 = time.perf_counter()
visited2 = visit('AA', 26, 0, 0, {})
total2 = max(v1+v2 for k1, v1 in visited2.items()
                   for k2, v2 in visited2.items() if not k1 & k2)
print(f"completed part 1: {total1} in {end1 - start1}", total2)
