import sys

from nltk.corpus import wordnet


def best_wup(w1, w2):
    syns1 = wordnet.synsets(w1)
    syns2 = wordnet.synsets(w2)

    scores = [
        s1.wup_similarity(s2)
        for s1 in syns1
        for s2 in syns2
        if s1.wup_similarity(s2) is not None
    ]
    return max(scores) if scores else 0.0


for pair in sys.argv[1].split(";"):
    w1, w2 = pair.split(",")
    print(best_wup(w1, w2))
