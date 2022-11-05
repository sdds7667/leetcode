import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;

class Trie {
    char c;
    HashMap<Character, Trie> adjList;
    boolean end;

    public Trie() {
        c = '\0';
        adjList = new HashMap<>();
        end = false;
    }

    public Trie(char c) {
        adjList = new HashMap<>();
        end = false;
        this.c = c;
    }

    public static Trie from(String[] words) {
        var root = new Trie();
        for(var i : words) {
            var currentNode = root;
            var chArr = i.toCharArray();
            for(int c = 0, n = i.length(); c < n; c++) {
                var nextNode = currentNode.adjList.getOrDefault(chArr[c], null);
                if (nextNode == null) {
                    nextNode = new Trie(chArr[c]);
                    currentNode.adjList.put(chArr[c], nextNode);
                }
                currentNode = nextNode;
            }
            currentNode.end = true;
        }
        return root;
    }
}
class Solution {

    int[][] directions = new int[][]{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
    public List<String> findWords(char[][] board, String[] words) {
        // building the trie
        var trie = Trie.from(words);
        var wordsSet = new HashSet<String>();
        boolean[][] visited = new boolean[board.length][board[0].length];
        for(var i = 0; i < board.length; i++) 
            for(var j = 0; j < board[i].length; j++)
                buildWords(board, visited, wordsSet, trie, i, j, new StringBuilder());
        
        return new ArrayList<String>(wordsSet);
            
    }

    private void buildWords(char[][] board, boolean[][] visited, HashSet<String> wordsSet, Trie trie, int i, int j, StringBuilder stringBuilder) {
        if (i < 0 || i >= board.length || j < 0 || j > board[0].length || visited[i][j]) return;
        var nextNode = trie.adjList.getOrDefault(board[i][j], null);
        if (nextNode == null) return;
        visited[i][j] = true;
        stringBuilder.append(board[i][j]);
        if (nextNode.end == true)
            wordsSet.add(stringBuilder.toString());

        for(var dir: directions) 
            buildWords(board, visited, wordsSet, nextNode, i + dir[0], j + dir[1], stringBuilder);
         
        stringBuilder.setLength(stringBuilder.length() - 1);
        visited[i][j] = false;
    }

}