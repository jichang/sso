import { Injectable } from "@angular/core";
import { User } from "./model";

@Injectable({
  providedIn: "root"
})
export class SessionService {
  constructor() {}

  current() {
    const currUser = localStorage.getItem("currUser");
    if (currUser) {
      return {
        currUser: JSON.parse(currUser) as User
      };
    }

    return null;
  }
}
