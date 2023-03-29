import java.util.*;


class Solution {
    public int minJumps(int[] arr) {

        // bfs from first?
        var positions = new HashMap<Integer, ArrayList<Integer>>();

        for (int i = 0; i < arr.length; i++) {
            if (i > 0 && i < arr.length && arr[i-1] == arr[i] && arr[i] == arr[i+1]) continue;
            if (positions.containsKey(arr[i])) {
                positions.get(arr[i]).add(i);
            } else {
                var newArrayList = new ArrayList<Integer>();
                newArrayList.add(i);
                positions.put(arr[i], newArrayList);
            }

        }

        var queue = new ArrayDeque<Integer>();
        queue.add(arr.length - 1);
        HashSet<Integer> visited = new HashSet<>();
        int position = 0;
        while (!queue.isEmpty()) {
            for (int k = 0, n = queue.size(); k < n; k++) {
                var i = queue.poll();
                if (i == 0) return position ;
                if (visited.contains(i)) continue;
                visited.add(i);
                for (var j : positions.get(arr[i]))
                    if (!visited.contains(j))
                        queue.add(j);
                if (i < arr.length - 1) queue.add(i + 1);
                queue.add(i - 1);
            }
            position++;
        }

        return -1;


    }
}