---
title: Deep Natural Language Processing - Introduction to Fundamentals and Applications
aliases:
  - NLP Fundamentals
  - Deep NLP Introduction
  - Natural Language Processing Basics
tags:
  - computer-science/nlp
  - machine-learning/deep-learning
  - note/lecture
creation_date: 2025-09-29
last_modified: 2025-09-29
status: complete
---

> [!summary] **Document Summary**
> This note provides a foundational introduction to **Natural Language Processing (NLP)**, covering its objectives, techniques, and applications. It explores the evolution from traditional to deep NLP methods, key preprocessing steps, and advanced techniques like syntactic parsing and named entity recognition. Additionally, it discusses text representation, feature engineering, and practical use cases such as sentiment analysis and machine translation, emphasizing their integration in data science pipelines.

## Deep Natural Language Processing: Introduction to Fundamentals and Applications

### Lecture Information
- **Title**: Deep Natural Language Processing
- **Institution**: Politecnico di Torino
- **Subtitle**: Introduction to NLP Fundamentals
- **Professor**: Prof. Luca Cagliero
- **Department**: Dipartimento di Automatica e Informatica, Politecnico di Torino
- **License**: CC - BY - NC - ND
- **References**:
  - Manning C.D., Schuetze H. *Foundations of Statistical Natural Processing*. MIT Press, 1999. ISBN 9780262133609
  - *The Text Mining Handbook: Advanced Approaches in Analyzing Unstructured Data*. Ronen Feldman, James Sanger. 2007. ISBN: 9780521836579
  - *Introduction to Statistical Machine Learning*. Raghav Bashyal
  - *Mining of Massive Datasets*. Jure Leskovec, Anand Rajaraman, Jeffrey D. Ullman. 2nd Ed. Cambridge University Press, 2014. ISBN 978-1107077232
  - *Multimodal Abstractivearization for How2 Videos*. Shruti Palaskar, Jindřich Libovický, Spandana Gella, Florian Metze. https://arxiv.org/abs/1906.07901
  - *Summarize Dates First a paradigm shift in timeline summarization*. Moreno La Quatra, Luca Cagliero, Elena Baralis, Maurizio Montagnulo, Alberto Messina. ACM SIGIR 2021
  - *Integration Of Sup-Lexical Linguistic Models With Speech Recognition Using Shallow Parsing And Finite State Transducers*. Xiaolong Mou, Stephanie Seneff, Victor Zue. MIT Press, 2003
  - *Named Entity Recognition Stanford NER Software*. Jenny Rose. Stanford University, 2007
- **Affiliations and Acknowledgements**:
  - The author and his staff are currently members of the Database and Data Mining Group at Dipartimento di Automatic e Informatica (Politecnico di Torino) and of the SmartData interdepartmental centre
  - https://dbdmg.polito.it
  - https://smartdata.polito.it
- **License**: Attribution + Noncommercial + NoDerivatives

### Lecture Goals
The lecture provides a foundational introduction to [[Natural Language Processing|NLP]]. It offers an overview of the main objectives, key techniques, and practical application in NLP. Additionally, it describes the core fundamentals of traditional NLP, including the standard NLP pipeline and essential text preprocessing steps. This structured approach ensures learners grasp both theoretical concepts and their real-world implications. #level/beginner #note/process

### Natural Language Processing (NLP)
> [!definition] **Natural Language Processing (NLP)**
> Natural Language Processing (NLP) aims at enabling computers to process human languages intelligently. In essence, it bridges the gap between human communication and machine understanding.

- NLP maintains a tight connection between various fields in computer science and linguistics, such as:
  - [[Artificial Intelligence]]
  - [[Machine Learning]]
  - Cognitive science
  - Information processing
  - And others...
- NLP specifically studies the connection between machine perception and languages. It involves programming computers to process large natural language corpora in a fruitful manner, allowing machines to derive meaningful insights from text data. #note/definition
- **Examples of Applications**:
  - Language understanding, where systems comprehend the meaning behind sentences.
  - Machine translation, converting text from one language to another seamlessly.
  - Text generation, creating coherent and contextually appropriate responses.
  - And more... #note/example
- **Related Resource**: https://chat.openai.com/ (This tool demonstrates modern NLP in action through interactive conversations.)

To illustrate the NLP pipeline conceptually, consider the following flowchart showing a basic sequential workflow from input text to processed output:

mermaid
flowchart LR
    A["Raw Text Input"] --> B["Preprocessing: Cleaning & Tokenization"]
    B --> C{"Apply NLP Techniques"}
    C -->|"Rule-Based or ML"| D["Analysis: e.g., Sentiment or Translation"]
    C -->|"Deep Learning"| E["Advanced Processing: e.g., Context Vector"]
    D --> F["Output: Insights or Generated Text"]
    E --> F
### Traditional vs. Deep Natural Language Processing
Understanding the evolution from traditional to deep approaches is key to appreciating modern NLP advancements. #level/intermediate

- **Traditional NLP Techniques**:
  - These methods relied on ad hoc linguistic models, for example:
    - Syntactic rules to parse sentence structure.
    - Dictionaries for word meanings and mappings.
    - Text generation based on predefined templates.
    - And similar hand-crafted approaches...
  - Importantly, traditional NLP does not necessarily rely on automated learning; it often depends on expert-defined rules. #note/process
  
- **Deep NLP Techniques**:
  - In contrast, deep NLP leverages [[Machine Learning]] models to automate the learning process. This allows systems to learn patterns directly from data, improving flexibility and accuracy over time. #note/definition

The shift to deep techniques has revolutionized NLP by handling complex, context-dependent tasks more effectively.

### Related Concepts
#### Text Mining
> [!definition] **Text Mining**
> Text Mining is the process of deriving significant information from text. It primarily focuses on unstructured and semi-structured text sources, transforming raw data into actionable knowledge.

- Text Mining encompasses several interrelated fields:
  - [[Information Retrieval]] (IR), for finding relevant documents.
  - [[Machine Learning]] (ML), for predictive modeling on text.
  - [[Natural Language Processing|NLP]], for linguistic analysis.
  - Knowledge management (KM), for organizing and utilizing extracted insights. #note/definition

#### Natural Language Understanding (NLU) or Interpretation (NLI)
> [!definition] **Natural Language Understanding (NLU) or Interpretation (NLI)**
> NLU, or Natural Language Interpretation (NLI), is a subtopic of NLP that deals with machine reading comprehension using AI techniques. It enables machines to "understand" text in a human-like way.

- Key components include:
  - Text categorization, assigning topics or labels to content.
  - Entity Recognition and Disambiguation, identifying and clarifying named elements like people or places.
  - And further semantic analysis... #note/process

#### Natural Language Generation (NLG)
> [!definition] **Natural Language Generation (NLG)**
> NLG focuses on generating human-like text or speech from structured data or specific instructions. It reverses the understanding process by creating natural outputs.

- Encompasses tasks such as:
  - Summarization, condensing long texts into key points.
  - Machine translation, producing translated content.
  - Question Answering, formulating responses to queries.
  - And more... #note/example

### Overview of NLP Applications
NLP techniques are widely applied in knowledge discovery and decision support systems. They are commonly integrated as a core component of data science pipelines, enhancing the analysis of textual data. The focus areas include a range of practical tasks that demonstrate NLP's versatility. #level/intermediate

- **Focus Areas**:
  - Sentiment analysis, gauging emotional tone.
  - Text categorization, organizing content by labels.
  - Machine translation, bridging language barriers.
  - Question answering, providing direct responses.
  - Topic modelling, uncovering hidden themes.
  - Text summarization, creating concise overviews. #note/list

#### Sentiment Analysis
> [!definition] **Sentiment Analysis**
> Sentiment Analysis extracts the writer’s feelings, opinions, emotions, likes, or dislikes from text. It is also known as opinion mining and helps quantify subjective information.

- This process identifies opinions and human behavior from plain text, relying on either traditional NLP rules or [[Machine Learning]] models.
- **Use Cases**:
  - Hotel review analysis to assess customer satisfaction.
  - News trading to gauge market sentiment.
  - Hate speech detection for content moderation.
  - Advertisement placing based on user preferences.
  - And others... #note/example
- **Example of Sentiment Analyzer: VADER**:
  - VADER (Valence Aware Dictionary and sEntiment Reasoner) is a lexicon and rule-based tool for sentiment analysis, particularly effective for social media text.
  - **Resource**: https://github.com/cjhutto/vaderSentiment #info

> [!example] **Practical Example**
> Here is a simple Python example using VADER to analyze sentiment:

python
from vaderSentiment.vaderSentiment import SentimentIntensityAnalyzer

# Initialize the sentiment analyzer
analyzer = SentimentIntensityAnalyzer()

# Input text
text = "The phone is super cool."

# Compute sentiment scores
scores = analyzer.polarity_scores(text)

# Output: {'neg': 0.0, 'neu': 0.326, 'pos': 0.674, 'compound': 0.7351}
print(scores)
In this example, the compound score of 0.7351 indicates a strongly positive sentiment.

#### Text Categorization
> [!definition] **Text Categorization**
> Text Categorization assigns a predefined label to a document or text snippet. It is also known as text classification and organizes content systematically.

- In supervised approaches, the system learns from labeled documents or text snippets to classify new, unlabeled ones accurately.
- If multiple labels are allowed, it becomes multi-label text classification, enabling nuanced tagging.
- This task relies on traditional NLP rules or [[Machine Learning]] techniques. #note/definition
- **Use Cases**:
  - Spam detection to filter unwanted emails.
  - Ticket management for prioritizing support requests.
  - Automatic text annotation for database organization.
  - Document management in enterprise systems.
  - And more... #note/example

> [!example] **Practical Example**
> For instance, a simple table illustrates supervised text categorization outcomes:

| Text Snippet              | Predicted Label | Confidence Score |
|---------------------------|-----------------|------------------|
| "Great product, fast delivery" | Positive Review | 0.92            |
| "This is spam and irrelevant" | Spam           | 0.85            |
| "Technical issue with software" | Support Ticket | 0.78            |

#### Machine Translation
> [!definition] **Machine Translation**
> Machine Translation involves the automatic translation of text or speech from one language to another, facilitating cross-lingual communication.

- **Approaches**:
  - Rule- or dictionary-based methods for structured translations.
  - Statistical methods that learn from parallel corpora.
  - [[Deep Learning]] techniques for context-aware translations. #level/advanced
- **Resource**: memoq (latest access: April 2021)
- **Rule-based Machine Translation**:
  - This approach uses linguistic information from source and target languages via bilingual dictionaries and grammars to perform translations.
  - It is known as Knowledge-Based Machine Translation or the Classical MT Approach.
  - **Resource**: https://translate.google.com (latest access: April 2021) #info
- **Statistical Machine Translation**:
  - It employs statistical models derived from the analysis of bilingual text corpora.
  - **Key Ideas**:
    - Every sentence in one language is a possible translation of any sentence in the other language.
    - The most appropriate translation is the one assigned the highest probability by the system.
- **Neural Machine Translation (NMT)**:
  - In NMT, an encoder creates a fixed-size context vector from all hidden states of the input sequence.
  - The decoder then uses this context vector to generate the target sequence word by word.
  - Specifically, the decoder predicts the next word based on the context vector and its previous predictions, enabling more fluid and accurate translations. #note/process

> [!example] **Practical Example**
> Consider a numerical example for probability in statistical MT: If "Hello" in English has translations "Bonjour" (probability 0.7) and "Hola" (probability 0.3) in French/Spanish contexts, the system selects "Bonjour" for a French target.

#### Question Answering
> [!definition] **Question Answering**
> Question Answering is a specialized area of [[Information Retrieval]] that provides relevant answers to natural language questions, mimicking human query resolution.

- **Main Steps**:
  1. Question classification to identify the type (e.g., factual or opinion-based).
  2. Information Retrieval to fetch candidate sources.
  3. Answer extraction to pinpoint the precise response. #note/process
- **Key Properties**:
  - **Source**: Can draw from web pages, knowledge bases, social data collections, and more.
  - **Question Formulation**: Includes domain-specific queries, factoid questions, and others.
  - **Answer Type**: Ranges from a single word, sentence, paragraph, to even images or videos.
- **Practical Use Cases**:
  - Chatbots for customer service interactions.
  - Remote medical assistance for quick diagnostics.
  - And similar applications... #note/example
- **Open-Domain Question Answering**: This refers to systems that answer questions from a broad, unrestricted knowledge base without domain limitations (mentioned as a concept without further details).

#### Conversational AI
Conversational AI extends Question Answering by adapting it to ongoing dialogues. It considers the history of past interactions and contextual elements, preserving the hierarchy and order of events to maintain coherent conversations. #note/definition

#### Topic Modelling
> [!definition] **Topic Modelling**
> Topic Modelling is an unsupervised [[Machine Learning]] technique that detects key word or phrase patterns in document collections. It uncovers latent themes without explicit labeling.

- A topic model provides a statistical representation of abstract word groups and expressions that characterize sets of documents.
- **Main Techniques**:
  - Latent Semantic Indexing (LSI), which uses singular value decomposition to reduce dimensionality and reveal relationships.
  - Latent Dirichlet Allocation (LDA), a probabilistic model assuming documents are mixtures of topics.
  - Aspect modelling, focusing on fine-grained sentiment or opinion aspects. #level/advanced
- **Practical Use Cases**:
  - [[Information Retrieval]] to improve search relevance.
  - Data visualization to map topics in corpora.
  - And more... #note/example
- **Dimensionality Reduction**:
  - This technique assumes data lies on or near a low-dimensional subspace.
  - The axes of this subspace provide an effective representation for the data, reducing noise and computational load. #note/process
- **Topic Modeling - BERTopic**:
  - BERTopic is a topic modeling technique that leverages BERT embeddings and c-TF-IDF for interpretable topics.
  - **Resource**: https://github.com/MaartenGr/BERTopic #info

> [!example] **Practical Example**
> Example: For a corpus of news articles, LDA might identify topics like "Politics" (keywords: election, vote) and "Technology" (keywords: AI, innovation).

#### Text Summarization
> [!definition] **Text Summarization**
> Text Summarization shortens large textual document collections to produce a concise summary of the salient content, preserving essential information while reducing length.

- **Input**:
  - A single document for focused overviews.
  - A multi-document collection for comparative summaries.
  - Multimodal data, incorporating text with images or videos.
- **Output**:
  - **Extractive Summary**: Selects and combines existing sentences or phrases from the input.
  - **Abstractive Summary**: Generates new sentences that reinterpret the content in a novel way. #note/definition
- **Approach**:
  - **Supervised**: Often uses [[Neural Networks]] for learning summary patterns, e.g., neural summarization models.
  - **Unsupervised**: Relies on methods like itemset mining, clustering, Latent Semantic Analysis, or graph ranking to identify key elements.
- **Language**:
  - Single-language for monolingual texts.
  - Multi-lingual for handling multiple languages.
  - Cross-lingual for translating and summarizing across languages.
- **Domain Specificity**:
  - General-purpose for broad applicability.
  - Context-dependent for specific fields like legal or medical texts.
  - Query-driven, tailored to user questions.
- **Time Dependency**:
  - Temporal summarization for event timelines.
  - Timeline summarization for chronological narratives.
  - Incremental summarization for updating summaries over time. #note/process
- **Practical Use Cases**:
  - Content curation to highlight key stories.
  - Accessibility for quick reading aids.
  - Learning analytics to summarize educational materials.
  - Highlight extraction for reports or articles. #note/example
- **Examples of Summarization Methods**:
  - TimeLine Summarization of news articles: *Summarize Dates First: a paradigm shift in timeline summarization*. Moreno La Quatra, Luca Cagliero, Elena Baralis, Maurizioagnulo, Alberto Messina. ACM SIGIR 2021

To visualize the summarization process, here is a flowchart:

mermaid
flowchart LR
    A["Input: Raw Documents"] --> B{"Supervised or Unsupervised?"}
    B -->|"Supervised"| C["Train Neural Model on Labeled Data"]
    B -->|"Unsupervised"| D["Apply Clustering or LSA"]
    C --> E["Generate Abstractive/Extractive Summary"]
    D --> E
    E --> F["Output: Concise Summary"]
### Text Structure
Text is divided into hierarchical units to facilitate automated processing and knowledge extraction. This structure varies based on the objective of analysis and the specific language involved. #level/intermediate

- **Different Aggregation Levels**:
  - Levels are chosen according to the analysis objective, such as semantic depth or syntactic parsing.
  - They also depend on language characteristics, like agglutinative vs. analytic structures. #note/process
- **Character**: The smallest text unit, including letters, digits, spaces, or special characters.
  - It has low utility on its own unless combined with positional information—for example, a space separates words, a full stop ends a sentence, and a question mark indicates an interrogative form. #note/definition
- **Word**:
  - A series of letters bounded by spaces, representing the smallest meaningful unit in the target language for the intended audience.
  - Processing words requires agreement on the language to handle variations like contractions or compounds. #note/definition
- **N-gram**:
  - A contiguous sequence of N textual units, such as phonemes, syllables, letters, or words.
  - While useful, n-grams do not guarantee syntactic or semantic relevance; they capture local patterns.
  - Specific types include unigram (1-gram, single word), bigram (2-gram, two words), and so on. #note/definition
  
> [!example] **Practical Example**
> **Example**: For the sentence "The quick brown fox", a bigram example is "quick brown". Here's a Python snippet to generate n-grams:

python
from nltk import ngrams
from nltk.tokenize import word_tokenize

# Sample text
text = "The quick brown fox jumps over the lazy dog."
 
# Tokenize into words
tokens = word_tokenize(text)
 
# Generate bigrams (N=2)
bigrams = list(ngrams(tokens, 2))
 
# Output: [('The', 'quick'), ('quick', 'brown'), ('brown', 'fox'), ...]
print(bigrams)
  - **Multi-word Expressions**:
  - These are textual forms consisting of at least two lexical items that function as a single unit, like "ice cream" or "New York".
- **Sentence**:
  - A text snippet separated by punctuation marks, such as full stops, question marks, or exclamation marks, conveying a complete thought. #note/definition
- **Phrase**:
  - A part of a sentence consisting of a group of words without a subject and verb—for example, the temporal phrase "after dinner" or the impersonal "waiting for the bus".
  - It does not convey a complete thought on its own. #note/definition
- **Clause**:
  - A part of a sentence that includes a subject and predicate.
  - It can stand alone as a complete sentence (independent clause, e.g., "the dog barked at him").
  - Or provide additional insights within a larger sentence (subordinate clause, e.g., "when the old man came"). #note/definition
- **Lexical Chain**:
  - A sequence of related words that span short distances (adjacent words or sentences) or long distances (across the entire text), linking concepts cohesively. #note/definition
- **Lemma**:
  - The canonical or base form of a word or multi-word expression, selected from dictionary candidates to represent inflected variants. #note/definition
- **Lexeme**:
  - A set of single or multi-word forms that share the same meaning.
  - The lemma serves as the conventional representative form—for example, handling the same word with different inflectional endings like "run", "runs", "running". #note/definition
- **Stem**:
  - The base form of a word, similar to a lemma but not necessarily derived from a dictionary.
  - It is obtained using static rules to strip inflected endings from words. #note/definition
- **Bag-Of-Word (BOW)**:
  - An unordered set of words from the text, with repetitions removed, treating the document as a collection of words ignoring order and grammar. #note/definition
- **Sectioning**:
  - **Paragraph**: A sequence of sentences that forms a coherent unit; it can be further partitioned into subparagraphs for finer granularity.
  - **Section**: A group of consecutive paragraphs, which can be subdivided into subsections or subsubsections for hierarchical organization. #note/definition
- **Examples**:
  - Text structure: examples of n-grams. Source: https://deepai.org/ (latest access: April 2021)
  - Text structure: examples of lexical chains. Source: https://Wikipedia.org (latest access: April 2021)
  - Text structure: examples of clauses and phrases. Source: https://knowitinfo.com/ (latest access: April 2021)

### Text Preprocessing
Text preprocessing prepares raw text for analysis by standardizing and cleaning it. This stage is crucial for improving the quality of downstream NLP tasks. #level/beginner #note/process

- **Taxonomy Reference**: *The Text Mining Handbook: Advanced Approaches in Analyzing Unstructured Data*. Ronen Feldman, James Sanger, 2007. ISBN: 9780521836579
- **Text Preprocessing Steps**:
  - **Cleaning**: Filters out noise, errors, and redundant content to ensure data purity.
  - **Tokenization**: Divides the raw text into smaller units or sub-units for granular processing.
  - **Stopword Elimination**: Removes frequent words that carry little semantic meaning, reducing noise.
  - **Part-Of-Speech Tagging**: Annotates words with their roles in the sentence for syntactic understanding.
  - **Lemmatization and Stemming**: Maps inflected or derivative forms back to their canonical base forms. #note/list
- **Language Dependence**:
  - Several steps are influenced by the language, including cleaning (partially), token, stopword elimination, Part-Of-Speech tagging, lemmatization, and stemming.
  - Handling unknown source languages requires additional detection mechanisms.

The following sequence diagram illustrates the interactions in a typical text preprocessing pipeline:

mermaid
sequenceDiagram
    "User Input"->>"Cleaning Module": "Raw Text"
    "Cleaning Module"->>"Tokenization": "Cleaned Text"
    "Tokenization"->>"Stopword Removal": "Tokens"
    "Stopword Removal"->>"POS Tagging": "Filtered Tokens"
    "POS Tagging"->>"Lemmatization/Stemming": "Tagged Tokens"
    "Lemmatization/Stemming"->>"User Input": "Processed Output"
#### Text Cleaning
- This step removes special characters, extra spaces, unrecognized symbols OCR errors, and other artifacts.
- **Case Normalization**: Standardizes text by converting to lowercase or uppercase to ensure consistency (e.g., "Apple" and "apple" treated the same). #note/process
- **Data Format Conversion**:
  - Converts formats like HTML, XML, JSON, or CSV into plain text.
  - May require parsing schemas to extract relevant content.
- It also handles non-textual content, such as embedded images or links, by stripping or converting them.

#### Tokenization
- Tokenization breaks the continuous stream of characters into meaningful constituents, such as words or sentences.
- It divides text into units or sub-units and identifies boundaries, often using punctuation, capitalization, or abbreviations.
- This process is strongly language-dependent, as rules vary (e.g., handling Chinese without spaces). #note/definition
- **Source**: https://kddnuggets.com (latest access: April 2021)
- **Example of Tokenizer**:
  - Source: https://keras.rstudio.com/reference/text_tokenizer.html (latest access: April 2021)

> [!example] **Practical Example**
> Example Python code using NLTK for sentence tokenization:

python
import nltk
from nltk.tokenize import sent_tokenize, word_tokenize

# Sample text
text = "This is a sample sentence. Here's another one!"

# Sentence tokenization
sentences = sent_tokenize(text)
print(sentences)  # Output: ['This is a sample sentence.', "Here's another one!"]

# Word tokenization
words = word_tokenize(text)
print(words)  # Output: ['This', 'is', 'a', 'sample', 'sentence', '.', 'Here', "'s", 'another', 'one', '!']
#### Stopword Elimination
> [!definition] **Stopword Elimination**
> Stop words are the most common words in a language that convey limited semantic information, such as prepositions, articles, and conjunctions (e.g., "the", "is", "and").

- These are typically filtered out before further processing to focus on content-bearing words.
- **Source**: https://kddnuggets.com (latest access: April 2021)
- The process is language-dependent, though some Deep NLP models omit it to preserve context.
- There is no universal stopword list; it is customizable based on the purpose—different search engines vary, and some even remove lexical words like "want" for performance optimization. #note/info
- **Example of Stopword List**:
  - Source: https://gist.github.com/sebleier/554280 (latest access: April 2021)
- **Example of Text Preprocessing Pipeline**:
  - Source: www.meaningcloud.com (latest access: April 2021)
- **Caveat**:
  - Removing words like "not" can alter the text's meaning (e.g., "not good" becomes "good", flipping sentiment). #warning
- For tasks requiring full semantics, it is better to deprecate stopword elimination and instead use Deep NLP techniques that handle context holistically.

#### Part-Of-Speech Tagging
- **Part-Of-Speech (POS)**: Refers to the category of a word based on its role in a sentence, such as noun, verb, or adjective. #note/definition
- **POS Tagging**: Involves labeling each word in the text with its appropriate POS tag (e.g., article, noun, verb, adjective, preposition, number, proper noun).
- Tagging is context- and language-dependent, relying on morphological analysis to disambiguate based on surrounding words.
- **Part-Of-Speech Tagging Example**:
  - Source: https://dataingovernment.blog.gov/ (latest access: April 2021)

> [!example] **Practical Example**
> Example: For "The cat runs quickly", tags might be: The (DT), cat (NN), runs (VBZ), quickly (RB).

#### Lemmatization
> [!definition] **Lemmatization**
> Lemmatization determines the lemma (base form) of a word by considering its intended meaning and part of speech.
  - It groups different inflected forms under a common lemma (e.g., "running", "runs" → "run").

- It is often performed alongside POS tagging and analyzes the context of the sentence or document (e.g., distinguishing "run" as noun vs. verb).
- Like POS tagging, it is context- and language-dependent, relying on morphological analysis and dictionaries. #note/process

#### Stemming
- Stemming is a simplified form of lemmatization that determines the stem (root form) of a word, rather than the exact lemma.
- It has lower complexity and provides an approximated result, which is acceptable in most information retrieval scenarios.
- **Source**: https://devopedia.org/ (latest access: April 2021) #info
- **Stemming vs. Lemmatization**:
  - Stemming is faster but cruder (e.g., "running" → "run", but "better" → "better" or "bet").
  - Lemmatization is more accurate but computationally intensive (e.g., "better" → "good").
  - Source: https://dataingovernment.blog.gov.uk/ (latest access: April 2021)

> [!example] **Practical Example**
> Python example comparing stemming and lemmatization using NLTK:

python
from nltk.stem import PorterStemmer, WordNetLemmatizer
from nltk import pos_tag, word_tokenize

# Sample words
words = ["running", "runs", "better", "geese"]

# Stemming
stemmer = PorterStemmer()
stems = [stemmer.stem(word) for word in words]
print("Stems:", stems)  # Output: ['run', 'run', 'better', 'gees']

# Lemmatization (requires POS for accuracy)
lemmatizer = WordNetLemmatizer()
lemmas = [lemmatizer.lemmatize(word) for word in words]
print("Lemmas:", lemmas)  # Output: ['running', 'run', 'better', 'goose'] (improves with POS)
### Advanced NLP Techniques
Advanced techniques build on preprocessing to perform deeper linguistic analysis, enabling more sophisticated applications. #level/advanced

#### Syntactic Parsing
- Syntactic Parsing conducts syntactical analysis based on a chosen grammar theory, revealing how words combine to form phrases and sentences.
- **Constituency Grammars**: These describe sentence structure through recursively built phrases, grouping syntactically related elements (e.g., noun phrases within a sentence tree). #note/definition
- **Dependency Grammars**: These focus on dependencies between words, such as an adjective modifying a noun, to model relationships.
- Parsing is computationally complex and often yields lower quality in real-world, ambiguous contexts due to variations in language use.
- **Parsing Example**: For the phrase “book that flight”, parsing might identify "book" as verb, "that flight" as object.
- **Syntactic Parsing**: (Diagram mentioned without details) #note/example

#### Shallow Parsing
- Shallow Parsing is a simplified version of full parsing that focuses on unambiguous snippets, such as simple noun or verb phrases.
- It trades off depth (full tree structure) for higher accuracy and efficiency in practical settings, avoiding the pitfalls of deep ambiguity. #note/definition

#### Named Entity Recognition (NER)
> [!definition] **Named Entity Recognition (NER)**
> Named Entity Recognition (NER) locates and classifies named entities in unstructured text into predefined categories.

- It identifies references to semantically rich concepts or bodies, often based on ontologies—for example, persons (e.g., "John Doe"), organizations (e.g., "Google"), locations (e.g., "Paris"), numeric expressions like time, date, money, or percent, and even domain-specific entities like proteins.
- **Reference**: *Named Entity Recognition and the Stanford NER Software*. Jenny Rose. Stanford University, 2007 #info

> [!example] **Practical Example**
> Example table of NER outputs:

| Text Snippet                  | Entity Type | Extracted Entity |
|-------------------------------|-------------|------------------|
| "Apple Inc. is based in Cupertino." | Organization | Apple Inc.      |
| "The meeting is on July 4th in Paris." | Location/Date | Paris / July 4th |
| "John bought $50 worth of shares." | Person/Money | John / $50      |

#### Word Sense Disambiguation
> [!definition] **Word Sense Disambiguation**
> Word Sense Disambiguation identifies the correct sense (meaning) of a word in a sentence when it has multiple possible interpretations (polysemy).

- **Approaches**:
  - Exploration of knowledge bases, like WordNet, to infer context.
  - [[Machine Learning]] techniques trained on sense-annotated data.
  - A combination of both for robust results. #level/advanced
- Acronym NERD: “Named Entity Recognition and Disambiguation” (highlights integration with NER).
- **Word Disambiguation Example: “cars”**:
  - The word "cars" can mean automobiles or a zodiac sign (Cancer in Italian "cancro", but context disambiguates).
  - Same Google query for "cars" might yield vehicle results, but context like "constellation cars" shifts to astronomical interpretations, leading to different search outcomes. #note/example

### Text Representation
> [!definition] **Text Representation**
> Text Representation transforms raw, unstructured text into a structured form suitable for [[Machine Learning]] or data mining algorithms. This conversion is essential because algorithms require fixed-sized, numerical inputs.

  - It demands a fixed-sized structured model, such as relational tables, key-value pairs, or equivalent vector representations.
- **Main Issue**: The challenge lies in converting unstructured or semi-structured text into a structured format without losing critical information. #note/info
- **Types of Documents**:
  - **Fully Unstructured Document**: Consists of raw text with minimal sectioning or paragraphs, like free-form emails.
  - **Weakly Structured Document**: Text organized into sections or paragraphs following a predefined format, such as reports with headings.
  - **Semi-structured Document**: Text overlaid with tags or markup, like XML or HTML, which provides partial structure for parsing. #note/definition
- **Feature-Value Data Model**:
  - This is a structured representation of key information extracted from text.
  - Features summarize text properties, such as the frequency of occurring words or syntactic patterns.
  - These are often derived from textual units like sentences or documents.
  - **Text Representation Example**: In column-based databases, text features can be stored as vectors in columns for efficient querying. #note/example
- **Main Steps**:
  - **Feature Engineering**: Involves defining relevant features and collecting their values using textual units and domain-specific heuristics to capture essence.
  - **Data Transformation**: Builds the final structured representation by incorporating the engineered features into a model ready for analysis. #note/process

#### Feature Engineering
- Feature Engineering is the process of selecting and creating features that best represent the text for a given task. #note/definition
- **Dimensionality Reduction**:
  - This addresses the "curse of dimensionality" in high-dimensional text data (e.g., vocabulary size in thousands) by reducing the number of features while retaining information.
  - **Main Methods**:
    - Latent Semantic Indexing (LSI): Uses matrix factorization to uncover latent topics.
    - Latent Dirichlet Allocation (LDA): Probabilistic modeling for topic-based reduction.
    - Principal Component Analysis (PCA): Linear transformation to lower dimensions, e.g., reducing a 10,000-feature vector to 100 principal components. #level/advanced

> [!math] **Mathematical Expression**
> Mathematical example for PCA: Given a feature matrix $X \in \mathbb{R}^{n \times p}$, compute covariance $C = \frac{1}{n} X^T X$, then eigenvectors for top-k dimensions. For $X = [1, 2; 3, 4]$, reduced form might project to a 1D line.

- **Feature Selection**:
  - This reduces the feature set to improve [[Machine Learning]] performance, such as in sentiment analysis, by eliminating irrelevant or redundant features.
  - **Unsupervised Methods**:
    - Ignore the target variable and remove redundancies, e.g., using Pearson correlation to drop highly correlated word features (correlation > 0.9).
  - **Supervised Methods**:
    - Leverage the target variable (e.g., sentiment label) to remove irrelevant features.
    - **Wrapper**: Exhaustively searches for well-performing feature subsets via model training (computationally intensive).
    - **Filter**: Selects subsets based on statistical relationships to the target, like chi-squared tests.
  - **Statistical Methods**: Use metrics like mutual information to rank features.
  - **Feature Importance Methods**:
    - **Intrinsic Methods**: Automatically select during training, e.g., Decision Trees that compute Gini impurity to prioritize splits on informative features. #note/process

## References
- [[Machine Learning]]
- [[Artificial Intelligence]]
- [[Neural Networks]]
- [[Linear Algebra]]