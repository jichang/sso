import { Component, OnInit } from "@angular/core";
import { Router } from "@angular/router";
import { session } from "../model";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import {
  PreferenceService,
  Preference,
  PREFERENCES
} from "../preference.service";
import { Subscription } from "rxjs";
import { MatSlideToggleChange } from "@angular/material/slide-toggle";
import { MatSnackBar } from "@angular/material/snack-bar";

@Component({
  selector: "settings-page",
  templateUrl: "./settings-page.component.html",
  styleUrls: ["./settings-page.component.css"]
})
export class SettingsPageComponent implements OnInit {
  PREFERENCES = PREFERENCES;
  preferences: {
    [index: number]: Preference;
  } = {};
  preferencesSubscription: Subscription;

  constructor(
    private router: Router,
    private http: HttpClient,
    private preferenceService: PreferenceService,
    private snackBar: MatSnackBar
  ) {}

  ngOnInit() {
    this.preferencesSubscription = this.preferenceService.preferences.subscribe(
      preferences => {
        this.preferences = {};
        for (let preference of preferences) {
          this.preferences[preference.key] = preference;
        }
      },
      err => {
        if (err.status === 403) {
          this.snackBar.open("action is forbidden", "Dismiss", {
            duration: 3000
          });
        }
      }
    );

    let currUser = session.currUser();
    if (currUser) {
      this.preferenceService.sync(currUser.id);
    } else {
      this.router.navigate(["signin"]);
    }
  }

  updatePreference(evt: MatSlideToggleChange, key: number) {
    let currUser = session.currUser();
    if (currUser) {
      this.preferenceService.update(currUser.id, {
        key,
        enabled: evt.checked,
        details: {}
      });
    } else {
      this.router.navigate(["signin"]);
    }
  }

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
      this.router.navigate(["/signin"]);
    });
  }
}
