class Solution {
    public List<Integer> getRow(int rowIndex) {
        var l = List.of(new ArrayList<Integer>(), new ArrayList<Integer>());
        l.get(0).add(1);l.get(1).add(1);l.get(1).add(1);
        
        if (rowIndex == 0) return l.get(0);
        if (rowIndex == 1) return l.get(1);
        
        int i = 0;
        int k = 1;
        
        for(int j = 2; j <= rowIndex; j++) {
            var li = l.get(i);
            var lk = l.get(k);
            li.clear();
            li.add(1);
            for(var m = 1; m < lk.size(); m++) 
                li.add(lk.get(m - 1) + lk.get(m));
            li.add(1);
            i = k;
            k = (k == 1) ? 0 : 1;
        }
        return l.get(k);
    }
}