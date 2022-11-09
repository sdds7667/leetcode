from typing import List

    

class StockSpanner:

    def __init__(self):
        self.xs = []
        self.ls = []

    def next(self, price: int) -> int:

        t = 1
        while len(self.xs) != 0 and self.xs[-1] <= price:
            t += self.ls.pop()
            self.xs.pop()
        self.ls.append(t)
        self.xs.append(price)
        return t