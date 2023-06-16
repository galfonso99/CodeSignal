
class Trie {
    map:{[key:string]:Trie} = {};
    isWord:boolean = false;
    constructor() {
    }

    public insert(word: string): void {
        	this.add(word,0,this)
           }

    private add(word:string,index:number,letterMap:Trie):void{
  	  	if(index == word.length){
    	  		letterMap.isWord = true
    	  		return; 
        	}
  		if(!letterMap.map[word.charAt(index)]){
    	  		letterMap.map[word.charAt(index)] = new Trie()
  		}
        	return this.add(word,index+1,letterMap.map[word.charAt(index)])
    	    }
}

function solution(words: string[], parts: string[]): string[] {
    let chunks = new Trie()
    for (let pf of parts) {
        chunks.insert(pf)
    }
    for (let [y,wr] of words.entries()) {
        let range = [0,0]
        for (let i=0; i<wr.length; i++) {
            let j = i
            let ref = chunks
            while (j < wr.length && ref.map[wr[j]]) {
                ref = ref.map[wr[j]]
                if (ref.isWord && j+1-i > range[1]-range[0]) {range = [i,j+1]}
                j++
            }
        }
        words[y] = braces(words[y], range[0], range[1])
    }
    return words
}
const braces = (str: string, s:number, l:number) : string => {
    if (s === l) return str
    return str.substring(0, s) + '[' + str.substring(s, l) + ']' + str.substring(l)
}