#### KMP Algorithm

##### Usage: String matching e.g. IndexOf() in JAVA
- Problem: 2 Strings, one is source string, one is target string. Try to figure out if the target string contains source string. If it does, return the first index of the matched source string in target string.
* Example: 
  Source String: aabaaf
  Target String: aabaabaafa

##### Idea (Kind of DP)
Try to memorize the parts of source String that has already checked, and avoid the repeated checking. In example, start matching from index 0, until approached index 5 'f'. If brute force, need to start the whole matching process again from the index 1. But it is easy to find that in "aabaaf" there is a pattern "aa" repeated, that means if the matching pass the second "aa", we just need to jump to the index after the first "aa" to restart the matching process instead of starting from the index 0. **The reason is, if passing the 2nd "aa", there must a "aa" in target String just before the unmatched index. We can regared that "aa" matching with the 1st "aa" in source String, to skip the repeated matching checking.** Therefore, what we need is a structure to memory this kind of repeated pattern, and also include the message to indicate where to jump back to restart the matching process. This structure is **NEXT ARRAY(Prefix Table).**

##### Next Array(Prefix Table)
- Longest equal prefix and surffix:
  - Prefix is all substring of a string which start with the char on index 0, without the char on index len-1
  - surffix is all substring of a string which end with the char on index len-1, without the char on index 0.
  - Example, for String aabaa:
    - Prefix: a, aa, aab, aaba
    - surffix: abaa, baa, aa, a
    - Longest equal one is "aa"
- Next Array, and how to create it
  - An array, each element indicate how long the "longest equal prefix and surffix" until this index. For example, if Next[3]=2, means including the char on index 3, the length of "longest eqaul prefix and surffix" is 2.
  - Use 2 pointers i, j, i point to the end of the surffix and used as the index to iterate the source String, j point to the end of the prefix.
    ```
    j=0
    Next[0]=j
    i=1
    if source_string[i]==source_string[j]:
        next[j] = 1
    else 
        next[j] = 0
    i++
    Iterate the source_string from i to the end:
        // the added char to surffix does not equal to the last char of the longest eqauled prefix
        // go to prev element in Next Array which indicate the length of the longest equal prefix and surffix without current char as the end of prefix
        // set the last pointer j by above and do the comparison again until find the matching char of candidate longest equal prefix, or cannot found any(the longest one is length 0)
        while j>0 and source_string[j]!=source_string[i]:
            j = Next[j-1]
        
        // the added char to the surffix equal to the last char of longest equaled prefix
        // therefore just extend the length of longest equaled prefix and surffix by 1
        // Also go to the next char of the prefix 
        if source_string[i]==source_string[j]: 
            j++
        
        // store the length
        next[i] = j

        // go to the next char of source string
    ```

##### Use the Next Array
The whole process is similar to the Next Array creating. Set 2 pointer, i and j. Pointer i is the index to iterate the target String, and point j is the index of Next Array(and also the index of source String). If the problem is to create a function like IndexOf() in JAVA, just need to break the iteration of target String when j==source_string.len().
**The reason is we can regard the whole source String as a Prefix, and we need to found the matching surffix in target String.**