from itertools import product
from typing import Tuple

# py3.8写法 Tuple[str, ...]; py3.9写法 tuple[str] 即不用导入 Tuple
def is_valid(answers: Tuple[str]) -> bool:
    # 将答案映射到题目中 题目从 1 开始编号
    q1, q2, q3, q4, q5, q6, q7, q8, q9, q10 = answers
    # q_list = list(answers)

    # 规则 1: 第1题的答案是 A. A B. B C. C D. D
    # if q1 not in 'ABCD':
    #     return False

    # 规则 2: 第5题的答案是 A. C B. D C. A D. B
    if q5 != {'A': 'C', 'B': 'D', 'C': 'A', 'D': 'B'}[q2]:
        return False

    # 规则 3: 以下选项中哪一题的答案与其他三项不同
    # q3_dict = { 'A': q3, 'B': q6, 'C': q2, 'D': q4 }
    def existentiallySame():
        if q3=='A' and (q3==q6 or q3==q2 or q3==q4):
            return True
        elif q3=='B' and (q6==q3 or q6==q2 or q6==q4):
            return True
        elif q3=='C' and (q2==q3 or q2==q6 or q2==q4):
            return True
        elif q3=='D' and (q4==q3 or q4==q6 or q4==q2):
            return True
        else:
            return False
    if existentiallySame():
        return False

    # 规则 4: 以下选项中哪两题的答案相同
    q4_dict = { "A": (q1, q5), "B": (q2, q7), "C": (q1, q9), "D": (q6, q10)}
    def both_questions_not_same(qn1:str,qn2:str):
        return not qn1==qn2
    if both_questions_not_same(*q4_dict[q4]):
        return False

    # 规则 5: 以下选项中哪一题的答案与本题相同
    q5_dict = {"A": q8, "B": q4, "C": q9, "D": q7}
    if q5_dict[q5]!=q5:
        return False
    
    # 规则 6: 以下选项中哪两题的答案与第8题相同
    q6_dict = { "A": (q2,q4), "B": (q1, q6), "C": (q3, q10), "D": (q5, q9)}
    def not_eq8(qn1:str,qn2:str):
        return not qn1==qn2==q8
    if not_eq8(*q6_dict[q6]):
        return False

    # 规则 7: 在这十道题中，被选中次数最少的选项字母为
    # tip: 这里可以只比较值, 因为只要拿到最小值, 然后对应的 key 都可以称为次数最少的选项字母
    counts = {c: answers.count(c) for c in 'ABCD'}
    q7_dict = {'A': "C", "B": "B", "C": "A", "D": "D"}
    if min(counts.values()) != counts[q7_dict[q7]]:
        return False

    # 规则 8: 以下选项中哪一题的答案与第1题的答案在字母中不相邻
    q8_dict = {'A': q7, "B": q5, "C": q2, "D": q10}
    def isAdjacent(qn:str):
        return abs(ord(q1)-ord(qn))==1
    if isAdjacent(q8_dict[q8]):
        return False

    # 规则 9: 已知“第1题与第6题的答案相同”与“第X题与第5题的答案相同”的真假性相反
    q9_dict = {'A': q6, "B": q10, "C": q2, "D": q9}
    is_q1_eq_q6 = q1 == q6
    def isSame(qn:str):
        return is_q1_eq_q6 == (qn == q5)
    
    if isSame(q9_dict[q9]):
        return False

    # 规则 10: 在这10道题的答案中，ABCD四个字母出现次数最多与最少者的差为
    d = max(counts.values()) - min(counts.values())
    q10_dict = {'A': 3, "B": 2, "C": 4, "D": 1}
    if d != q10_dict[q10]:
        return False

    return True

# Enumerate 所有可能的答案组合
for answers in product('ABCD', repeat=10):
    if is_valid(answers):
        print(' '.join(answers))
        break