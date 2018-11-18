import { Component, OnInit } from "@angular/core";
import { Router } from "@angular/router";
import { session } from "../model";
import { HttpClient, HttpHeaders } from "@angular/common/http";

@Component({
  selector: "settings-page",
  templateUrl: "./settings-page.component.html",
  styleUrls: ["./settings-page.component.css"]
})
export class SettingsPageComponent implements OnInit {
  constructor(private router: Router, private http: HttpClient) {}

  ngOnInit() {}

  signOut() {
    let headers = new HttpHeaders({
      Authorization: "Bearer " + window.localStorage.getItem("jwt")
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/signout`;

    return this.http.post(apiUri, options).subscribe(() => {
      session.destory();
      this.router.navigate(["/"]);
    });
  }
}
