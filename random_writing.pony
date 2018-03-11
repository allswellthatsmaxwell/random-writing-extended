use "options"
use "collections"
use "random"
use "files"

type CharCounter is (Map[U8, I64])
type CharProba is (Map[U8, F64])
type KCharCounts is (Map[String, CharCounter])
type KCharDistribution is (Map[String, CharProba])

actor Main
  let _env: Env
  var _src: String = ""
  var _len: I64 = 0
  var _k: I64 = 5 
  
  new create(env: Env) =>
    _env = env
    try
      arguments()?
      with file = OpenFile(FilePath(env.root as AmbientAuth, _src)?) as File
      do
        let text = recover val file.read_string(file.size()) end
        env.out.print(text)
        let distribution = get_distribution(text, _k)?
        print_distribution(distribution)
      end      
    end

  fun counts_to_probabilities(prefix_counts: CharCounter): CharProba =>
    var prefix_probas = KCharDistribution()
    for prefix in prefix_counts.keys() do
      var char_probas = prefix_probas.insert(prefix, CharProba())?
      let char_counts = prefix_counts(prefix)
      let n_instances = char_counts.values().sum()
      for count in char_counts do
        char_probas.insert(count / n_instances)?
      end
    end
  
    
  fun print_distribution(distribution: KCharCounts): None =>
    for (prefix, counts) in distribution.pairs() do
      for (char, count) in counts.pairs() do
        _env.out.print(prefix + "| " + String.from_array([char]) + ": " + count.string())
      end
    end

    
  fun get_distribution(text: String, k: I64): KCharCounts ? =>
    let avail_len = text.size() - k.usize()
    var distribution = KCharCounts()
    for loc in Range(0, avail_len - 1) do
      let k_string = recover val text.substring(loc.isize(), loc.isize() + k.isize()) end
      let next = text(loc + k.usize()) ?
      var prefix_map = distribution.insert_if_absent(k_string, CharCounter())?
      prefix_map.upsert(next, 1, {(x, y) => x + y})?      
    end
    distribution

    
  fun ref arguments(): None ? =>
    var options = Options(_env.args)
    options.add("file", "f", StringArgument)
    options.add("len", "l", I64Argument)
    options.add("k", "k", I64Argument)
    for o in options do
      match o
        | ("file", let arg: String) => _src = arg
        | ("len", let arg: I64) => _len = arg
        | ("k", let arg: I64) => _k = arg
        | let err: ParseError => err.report(_env.out); usage(); error
      end
    end

  fun ref usage(): None =>
    _env.out.print(
    """
    random_writing [OPTIONS]
    --src  :  the path to a file of text to read
    --len  :  the length of the output
    --k    :  the character length of the seed
    """)
