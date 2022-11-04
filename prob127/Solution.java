import java.util.ArrayList;
import java.util.List;

class Solution {
    public int ladderLength(String beginWord, String endWord, List<String> wordList) {
        
        int start_index = wordList.size();
        int end_index = wordList.size();
        for(int i = 0; i < wordList.size(); i++) {
            if (wordList.get(i).equals(beginWord)) 
                start_index = i;
            else if (wordList.get(i).equals(endWord)) {
                end_index = i;
            }
        }
        if (end_index == wordList.size()) return 0;
        if (start_index == wordList.size())
            wordList.add(beginWord);
        System.out.println("" + start_index + " " + end_index);
        return bfs(buildAdjacencyList(wordList), start_index, end_index);
    }
    

    public int bfs(ArrayList<ArrayList<Integer>> adjacencyList, int start, int finish) {
        ArrayList<Integer> queue = new ArrayList<Integer>();
        boolean[] visited = new boolean[adjacencyList.size()];
        queue.add(start);
        visited[start] = true;
        int size = 1;
        while(!queue.isEmpty()) {
            var newQueue = new ArrayList<Integer>();
            for(var i: queue) {
                for(var child: adjacencyList.get(i)){ 
                    if (child == finish) return size + 1;
                    if (visited[child]) continue;
                    visited[child] = true;
                    newQueue.add(child);
                }
            }
            queue = newQueue;
            size++;
        }
        return 0;
    }

    public boolean canLadderFrom(String l, String r) {
        int diff = 0;
        for(int i = 0, n = l.length(); i < n && diff < 2; i++) 
            if (l.charAt(i) != r.charAt(i)) 
                diff++;
        return diff == 1;
    }

    public ArrayList<ArrayList<Integer>> buildAdjacencyList(List<String> wordList) {
        int n = wordList.size();
        ArrayList<ArrayList<Integer>> adjacencyList = new ArrayList<ArrayList<Integer>>(wordList.size());
        for(var i = 0; i < n; i++) adjacencyList.add(new ArrayList<Integer>());
        for(var i = 0; i < n; i++)
            for(var j = i + 1; j < n; j++)
                if (canLadderFrom(wordList.get(i), wordList.get(j))) {
                    // System.out.println("Can ladder from " + wordList.get(i) + " " + wordList.get(j));
                    adjacencyList.get(i).add(j);
                    adjacencyList.get(j).add(i);
                }
        return adjacencyList;
    }
}