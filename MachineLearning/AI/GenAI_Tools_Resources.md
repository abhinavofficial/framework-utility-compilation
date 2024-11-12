# GenAI Tools and Resources

## Gemini

### Google Gemini API

- [Quickstart(]https://ai.google.dev/gemini-api/docs/quickstart)

### Google AI Studio

[Google API Collab](https://colab.research.google.com/)

### Prompts

1. Freeform Prompt: A freeform prompt is an open-ended input method that allows users to interact naturally with AI Models, fostering creativity and flexibility in generating tailored responses without strict constraints.
2. Structured Prompt: A structured prompt is a specific input format that provides clear guidelines or templates for users to interact with AI Models. Unlike freeform prompts, structured prompts require users to input information in a predefined format, enabling more controlled and targeted interactions with AI systems.
3. Conversational Prompt: A conversation prompt is a type of input that simulates a natural conversation between a user and an AI model. This prompt format allows users to engage with AI systems in a converational manner, where the AI responds contextually to the user's queries or statements, creating a more interactive and human-line interaction experience.

### Gemini API

The Gemini API is a powerful tool provided by Google that grants access to cutting-edge generative models, enabling developers to leverage advanced AI capabilities for various applications. This API allows users to interact with Gemini models, which are multimodal generative AI models capable of processing text and image inputs to generate text responses.

### Settings

- **Token count**: A token is commonly around 4 characters: OpenAI (GPT) has an interactive tokeniser for GPT-3 and Codex here and claims a token is approximately 4 characters or 3/4ths of a word; Anthropic (Claude) also claims a token is approximately 3/4ths of a word. If if taken count is mentioned as 1MM, it means your prompt token + response token should be less that 1MM. Please note that in case of sound and video as well, the concept of token applies. Here is a sample code to get the token count.

```python
import vertexai
from vertexai.generative_models import GenerativeModel, Part

# TODO(developer): Update and un-comment below lines
# project_id = "PROJECT_ID"

vertexai.init(project=project_id, location="us-central1")
model = GenerativeModel("gemini-1.5-flash-001")

# count token for a multimodal prompt
contents = [
    Part.from_uri(
        "gs://cloud-samples-data/generative-ai/video/pixel8.mp4",
        mime_type="video/mp4",
    ),
    "Provide a description of the video.",
]
response = model.count_tokens(contents)
print(f"Prompt Token Count: {response.total_tokens}")
print(f"Prompt Character Count: {response.total_billable_characters}")

# count token for Response
response = model.generate_content(contents)
usage_metadata = response.usage_metadata
print(f"Prompt Token Count: {usage_metadata.prompt_token_count}")
print(f"Candidates Token Count: {usage_metadata.candidates_token_count}")
print(f"Total Token Count: {usage_metadata.total_token_count}")
```

- **Temperature**: It is a reflection of how random you would like to the output to be. Higher temperature means the response should be more aligned to user prompt.
- **Add Stop Prompt**: If you want to stop the response token if this word is hit. ??
- **Safety**: You can block harassment, hate speech, sexually explicit, or dangerous content partially or fully.

### DALL.E

[DALL.E](https://openai.com/index/dall-e-3/) is an AI model developed by OpenAI that generates images from textual descriptions. It combines creativity with precision, allowing users to visualize ideas through AI-generated artwork.

[OpenAI developer platform](https://platform.openai.com/docs/overview)

### Tensorflow

Google's top-tier open-source machine learning framework, Tensorflow, provides a flexible ecosystem for creating and implementing AI models on a range of platforms. For those new to machine learning as well as seasoned practioners, Tensorflow is an excellent option. Although it supports languages including Javascript, C++, and Java, it predominantly uses Python.
