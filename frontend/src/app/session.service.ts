import { Injectable } from "@angular/core";

@Injectable({
  providedIn: "root"
})
export class SessionService {
  constructor() {}

  current() {
    const currUser = localStorage.getItem("currUser");
    if (currUser) {
      return;
    }

    return null;
  }
}
