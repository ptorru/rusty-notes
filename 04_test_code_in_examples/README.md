# Using code from tests in our examples

Today I am having a bit of an issue. I have made some code for my integration tests, and well, the core of it does exactly what I would like to expose as an example for my project. Ideally I would like to use this code, wrapped into a function, in both my integration test and my example code.

## Failure
Well, what i was trying to do was not quite right. I was trying to essentially share and expose code on multiple places the lazy way, eg, do some code in my testing area and expose that to examples. That may not be the best approach, as in general you would not want to do this. One of my motivations to do it this way was to avoid giving the impression edifier wants to offer certain facilities; yes, I would not want to give the impression for instance that we are aiming to support all/multiple platforms, this should come from somewhere else. However by trying to do two things:
* Avoid giving the wrong impression.
* Re-use all possible code trying to do "DRY" to the nth-degree.

I was structuring my code in a way is not quite friendly with cargo. Perhaps this is one learning to keep in mind, which, if there is a conventioin, you may have to bend your understanding of the world in order to fit that convention, in order to enjoy the facilities/benefits provided from following that convention.