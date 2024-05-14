# Online LLM
Online LLM in Rust with REST API compatible to OpenAIs API. 
The work is based on implementation of [FreshLLMs](https://arxiv.org/abs/2310.03214).
Its publicly hosted.
If you are curious to try it out send me an email and I will provide you with the link and API key.

## Usage
The Online-LLM can be reached through the standard OpenAI API.
To use it in Python, you can do:

```python
from openai import OpenAI

client = OpenAI(base_url=<URL_ON_REQUEST>, api_key=<API_KEY_ON_REQUEST>)

completion = client.chat.completions.create(
    model="gpt-4o",
    messages=[{"role": "user", "content": "What is the top news about Gen AI today?"} ]
)

print(completion.choices[0].message) 
```

## Example Outputs
[Outputs as of May 2024]
The authors of FreshLLMs distinguish four types of online questions: Never changing, slowly changing, fast changing and false premise

### False Premise: What was the text of Donald Trump’s ﬁrst tweet in 2022, made after his unbanning from Twitter by Elon Musk?
> Correct response by Online LLM, false response by GPT-4. 
<table>
  <tr>
    <td valign="top" style="width: 50%;">
      <strong>Online LLM:</strong> Donald Trump did not tweet in 2022 after his unbanning by Elon Musk. He remained off Twitter for that year. He made his first tweet on the platform, now known as X (formerly Twitter), in 2023. His first tweet after reinstatement was of his mugshot.
      Sources:
      <ul>
        <li><a href="https://www.nbcnews.com/tech/tech-news/trump-tweets-first-time-2021-show-mug-shot-rcna101736" title="NBC News article on Trump's tweet">NBC News</a></li>
        <li><a href="https://www.washingtonpost.com/technology/2023/08/24/trump-back-twitter-mugshot/" title="Washington Post article on Trump's tweet">Washington Post</a></li>
        <li><a href="https://www.cbsnews.com/news/donald-trump-mug-shot-tweet-twitter-x-return/" title="CBS News article on Trump's tweet">CBS News</a></li>
      </ul>
    </td>
    <td valign="top" style="width: 50%;">
      <strong>Plain GPT-4:</strong> As of my last update, Donald Trump was banned from Twitter in January 2021 and has not been reinstated by Twitter, even after its acquisition by Elon Musk. Consequently, Donald Trump did not have a first tweet in 2022 because he was not active on the platform during that year.
    </td>
  </tr>
</table>

### Fast Changing: Which game won the Spiel des Jahres award most recently?
> Correct response by Online LLM, false response by GPT-4. 
<table>
  <tr>
    <td valign="top" style="width: 50%;">
      <strong>Online LLM:</strong> The most recent winner of the Spiel des Jahres award is "Dorfromantik: The Board Game." It won the award in 2023.
      Sources: 
        <ul>
        <li><a href="https://www.beastsofwar.com/board-games/dorfromantik-spiel-des-jahres-2023-winners-announced/" title="Beasts of War article on Dorfromantik">Beasts of War</a></li>
        <li><a href="https://pegasusna.com/news/dorfromantik-the-board-game-awarded-2023-spiel-des-jahres" title="Pegasus Spiele news on Dorfromantik">Pegasus Spiele</a></li>
        <li><a href="https://www.gamesradar.com/dorfromantik-wins-board-game-of-the-year-in-spiel-des-jahres-2023/" title="Games Radar article on Dorfromantik">Games Radar</a></li>
        <li><a href="https://www.polygon.com/tabletop-games/23798614/spiel-des-jahres-2023-dorfromantik-the-board-game" title="Polygon article on Dorfromantik">Polygon</a></li>
      </ul>
    </td>
    </td>
    <td valign="top" style="width: 50%;">
      <strong>Plain GPT-4:</strong> As of my last update, the most recent game to win the Spiel des Jahres award is "Cascadia," which won in 2023. This game is a tile-laying and token-drafting game focused on wildlife and habitats.
    </td>
  </tr>
</table>

### Slow Changing: Which team holds the record for largest deﬁcit overcome to win an NFL game?
> Correct response by Online LLM, false response by GPT-4. 
<table>
  <tr>
    <td valign="top" style="width: 50%;">
      <strong>Online LLM:</strong> The Minnesota Vikings hold the record for the largest deficit overcome to win an NFL game. They overcame a 33-0 halftime deficit to defeat the Denver Broncos with a final score of 39-36. 
      Source: <a href="https://en.wikipedia.org/wiki/List_of_largest_comebacks_in_NFL_games" title="Wikipedia: Largest comebacks in NFL games">Wikipedia: Largest comebacks in NFL games</a>
    </td>
    </td>
    <td valign="top" style="width: 50%;">
      <strong>Plain GPT-4:</strong> The record for the largest deficit overcome to win an NFL game is held by the San Francisco 49ers. 
      They came back from a 35-point deficit to defeat the New Orleans Saints. This historic comeback occurred during a game on December 7, 1980, when the 49ers were trailing 35-7 at halftime but managed to win the game with a final score of 38-35.
    </td>
  </tr>
</table>


### Never Changing: Has Virginia Woolf's novel about the Ramsay family entered the public domain in the United States?
> Correct answer by both.
<table>
  <tr>
    <td valign="top">
      <strong>Online LLM:</strong> Yes, Virginia Woolf's novel "To the Lighthouse" has entered the public domain in the United States. You can find free ebook downloads of the novel and access it without copyright restrictions.
      Sources:
      <ul>
        <li><a href="https://standardebooks.org/ebooks/virginia-woolf/to-the-lighthouse" title="StandardeBooks - To the Lighthouse by Virginia Woolf">StandardeBooks - To the Lighthouse by Virginia Woolf</a></li>
        <li><a href="https://archive.org/details/to-the-lighthouse-1927" title="Internet Archive - To the Lighthouse by Virginia Woolf">Internet Archive - To the Lighthouse by Virginia Woolf</a></li>
        <li><a href="https://babel.hathitrust.org/cgi/pt?id=uc1.31822042948885" title="HathiTrust Digital Library - To the Lighthouse by Virginia Woolf">HathiTrust Digital Library - To the Lighthouse by Virginia Woolf</a></li>
      </ul>
    </td>
    <td valign="top">
      <strong>Plain GPT-4:</strong> Yes, Virginia Woolf's novel "To the Lighthouse," which centers on the Ramsay family, has entered the public domain in the United States. The novel was originally published in 1927, and since works published before 1924 are in the public domain under U.S. law, "To the Lighthouse" became public domain on January 1, 2023, following the rule that places works into the public domain 95 years after publication.
    </td>
  </tr>
</table>

