import { combineReducers } from 'redux';
import exampleReducer from './features/example/exampleSlice';

const rootReducer = combineReducers({
  example: exampleReducer,
});

export default rootReducer;