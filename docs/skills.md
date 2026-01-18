# Polyglot-Agent OS: Skills

## Skill: AnalyzeLinguisticUnit
- **Description**: Analyzes a word or phrase for JP/EN/ZH.
- **Tools**: 
  - `tokenize(text, lang)`: Returns tokens with POS tags.
  - `get_han_viet(kanji)`: Maps Japanese/Chinese characters to Vietnamese-Sino phonetics.
  - `calculate_srs(object_id, quality)`: Updates memory stability using $R = e^{-\frac{t}{S}}$.

## Skill: IngestContent
- **Description**: Processes raw content into learning objects.
- **Tools**:
  - `extract_text(source_url)`: Scrapes text from web/PDF.
  - `generate_flashcards(text, level)`: Uses LLM to create Q&A pairs.
  - `push_to_kafka(topic, payload)`: Enqueues tasks for background processing.