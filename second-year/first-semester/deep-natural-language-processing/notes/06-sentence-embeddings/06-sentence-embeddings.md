---
title: Sentence Embeddings in Natural Language Processing
aliases: ["Sentence Embeddings", "Natural Language Processing", "NLP"]
tags: [technology/nlp, concept/representation, type/note]
creation_date: 2025-10-10
last_modified: 2025-10-10
status: complete
---
> [!summary] **Document Summary**
> This note explores the concept of sentence embeddings in natural language processing, discussing their purpose, methods for derivation, key techniques, and applications. It covers both unsupervised and supervised approaches, including models like InferSent, Skip-Thought Vectors, and the Universal Sentence Encoder, as well as evaluation criteria and references.

## Sentence Embeddings in Natural Language Processing

### Introduction to Sentence Embeddings

- **Definition**: Sentence Embeddings are vector representations of sentences that capture their semantic meaning.
- **Purpose**: To enable machines to understand and process natural language by converting sentences into numerical vectors.

### Word to Sentence Representation

- **From Word to Sentences**: The process involves transforming individual words into sentences and then into paragraphs.
- **Word - Document Representation**: 
  - Insensitive to word ordering.
  - Disregards word repetitions.

### Methods for Deriving Sentence Vectors

1. **Infer Word Vectors**: 
   - Derive sentence vectors from word vectors.
   - E.g., feature averaging.

2. **Unsupervised Sentence Encoding**: 
   - Learn generic, distributed sentence encoding from plain text.
   - E.g., `Doc2Vec`, `Sent2Vec`, `Skip-Thought`.
   - **Transfer learning paradigm**.

3. **Supervised Sentence Encoding**: 
   - Learn sentence encodings from labeled data.

### Key Concepts and Techniques

- **Distributed Representations of Sentences and Documents**: 
  - Proposed by `Quoc V. Le` and `Tomás Mikolov` in `ICML 2014`.
  - Sentences are represented by a column in a matrix `D`.
  - Words are represented by a column in matrix `W`.
  - It acts as a memory that remembers either what is missing from the current context or the topic of the paragraph.

- **Predict the Next Word Given the Context**: 
  - The context words are the preceding words only.
  - The surrounding context is encoded by the paragraph id.

- **Stochastic Gradient Descent**: 
  - At each iteration, a text window is sampled.
  - A single random word is sampled from that window, forming the classification task.

- **Sentence Representation Using Compositional n-Gram Features**: 
  - Proposed by `Matteo Pagliardini`, `Prakhar Gupta`, and `Martin Jaggi` in `NAACL 2018`.
  - The entire sentence is considered as the context window, instead of sampling a fixed-size context window.

- **Skip-Thought Vectors**: 
  - Proposed by `Ryan Kiros`, `Yukun Zhu`, `Ruslan R. Salakhutdinov`, `Richard Zemel`, `Raquel Urtasun`, `Antonio Torralba`, and `Sanja Fidler` in `NIPS 2015`.
  - Sentences that share semantic and syntactic properties are mapped to similar vector representations.

- **InferSent**: 
  - A model for supervised learning of universal sentence representations.
  - Trained on 570k human-generated English sentence pairs.
  - Manually labeled as `entailment`, `contradiction`, or `neutral`.

### Semantic Relationship Tasks

- **Determine Whether a “Hypothesis” Is True, False, or Neutral, Given a “Premise”**:
  - Involves high-level reasoning about semantic relationships within sentences.

- **Example**:
  - **Text**: "If you help the needy, God will reward you."
  - **Hypothesis**: "Giving money to a poor man has good consequences."
  - **Hypothesis**: "Giving money to a poor man has no consequences."
  - **Hypothesis**: "Giving money to a poor man will make you a better person."

### Neural Network Approaches

- **BiLSTM (Bidirectional Long Short-Term Memory)**:
  - Computes a set of T vectors { h t }.
  - For t ∈ [1, . . . , T], h t is the concatenation of a forward LSTM and a backward LSTM that read the sentences in two opposite directions.

- **Attention Mechanism**:
  - Uses an attention mechanism over the hidden states of a BiLSTM to generate a representation u of an input sentence.
  - { h 1 , . . . , h T } are the output hidden vectors of a BiLSTM.
  - tanh() is an affine transformation that outputs a set of keys.
  - The { α i } represents the score of similarity between the keys and a learned context query vector u w.
  - The final sentence representation u is a weighted linear combination of the hidden vectors.

- **Multiple Views of the Input Sentence**:
  - The model can learn which part of the sentence is important for the given task.
  - 4 context vectors u w which generate 4 representations that are then concatenated to obtain the sentence representation u.

### Universal Sentence Encoder

- **Definition**: A model for generating sentence embeddings.
- **Proposed by**: `Daniel Cera`, `Yinfei Yang`, `Sheng-Yi Kong`, `Nan Hua`, `Nicole Limtiacob`, `Rhomni St. John`, `Noah Constant`, `Mario Guajardo-Cespedes`, `Steve Yuan`, `Chris Tara`, `Yun-Hsuan Sung`, `Brian Strope`, `Ray Kurzweil`.
- **Implementation**: `Universal Sentence Encoder` available at `https://tfhub.dev/google/universal-sentence-encoder/1`.

### Evaluation and Performance

- **Fast to Compute Performance**: 
  - Efficient computation of sentence embeddings.
- **Needs Positive Correlation with Real Task**: 
  - To determine the usefulness of the embeddings.

### References

- `Quoc V. Le, Tomás Mikolov. Distributed Representations of Sentences and Documents. ICML 2014: 1188 - 1196`
- `Matteo Pagliardini, Prakhar Gupta, Martin Jaggi. Unsupervised Learning of Sentence Embeddings using Compositional n-Gram Features. NAACL 2018`
- `Ryan Kiros, Yukun Zhu, Ruslan R. Salakhutdinov, Richard Zemel, Raquel Urtasun, Antonio Torralba, and Sanja Fidler. 2015. Skip-Thought Vectors. In Advances in neural information processing systems (NIPS), pages 3294 – 3302.`
- `Alexis Conneau, Douwe Kiela, Holger Schwenk, Loic Barrault, Antoine Bordes. Supervised Learning of Universal Sentence Representations from Natural Language Inference Data. EMNLP 2017`
- `Daniel Cera, Yinfei Yang, Sheng-Yi Kong, Nan Hua, Nicole Limtiacob, Rhomni St. John, Noah Constant, Mario Guajardo-Cespedes, Steve Yuan, Chris Tara, Yun-Hsuan Sung, Brian Strope, Ray Kurzweil. Universal Sentence Encoder.`
- `https://www.topbots.com/ (latest access: June 2021)`
- `https://cs.stanford.edu/~quocle/paragraph_vector.pdf`
- `https://github.com/epfml/sent2vec`
- `https://github.com/facebookresearch/InferSent`