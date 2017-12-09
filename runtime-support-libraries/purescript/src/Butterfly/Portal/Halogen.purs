module Butterfly.Portal.Halogen
  ( State
  , Query
  , Input
  , Output
  , ui
  , initialState
  , render
  , eval
  , receiver
  ) where

import Prelude

import Control.Monad.Trans.Class (lift)
import Data.Lens (view)
import Data.Maybe (Maybe (..))
import Halogen.Component (Component, ComponentDSL, ComponentHTML, component)
import Halogen.HTML (HTML)

import Control.Monad.State.Class as State
import Data.Array as Array
import Data.Set as Set
import Halogen.HTML as HH
import Halogen.HTML.Events as HE

import Butterfly.Actor (Actor)
import Butterfly.Portal (Button (..), Portal (..), buttonActors)

type State = Actor
data Query f a
  = ChangeActor Actor a
  | ClickButton (f Unit) a
type Input = Actor
type Output = Void

ui :: ∀ f. Monad f => Portal f -> Component HTML (Query f) Input Output f
ui portal = component
  { initialState
  , render: render portal
  , eval
  , receiver }

initialState :: Input -> State
initialState = id

render :: ∀ f. Portal f -> State -> ComponentHTML (Query f)
render (Portal buttons) actor =
  HH.div [] (renderButton <$> buttons')
  where
    buttons' :: Array (Button f)
    buttons' = buttons
      # Array.fromFoldable
      # Array.filter (Set.member actor <<< view buttonActors)

    renderButton :: Button f -> ComponentHTML (Query f)
    renderButton (Button label _ action) =
      HH.button [HE.onClick <<< HE.input_ $ ClickButton action]
                [HH.text label]

eval :: ∀ f. Monad f => Query f ~> ComponentDSL State (Query f) Output f
eval (ChangeActor actor next) = next <$ State.put actor
eval (ClickButton action next) = next <$ lift action

receiver :: ∀ f. Input -> Maybe (Query f Unit)
receiver = Just <<< (ChangeActor <@> unit)
