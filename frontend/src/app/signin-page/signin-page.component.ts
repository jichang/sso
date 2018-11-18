import { Component, OnInit } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import * as model from "../model";
import { HttpErrorResponse } from "@angular/common/http";
import { MatSnackBar } from "@angular/material";

@Component({
  selector: "signin-page",
  templateUrl: "./signin-page.component.html",
  styleUrls: ["./signin-page.component.css"]
})
export class SigninPageComponent implements OnInit {
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private snackbar: MatSnackBar
  ) {}

  ngOnInit() {}

  signined({ jwt, user }: { jwt: string; user: model.User }) {
    window.localStorage.setItem("jwt", jwt);
    window.localStorage.setItem("currUser", JSON.stringify(user));

    this.route.queryParamMap.subscribe(params => {
      const redirectUrl = params.get("redirectUrl");
      if (redirectUrl) {
        this.router.navigate([redirectUrl]);
      } else {
        this.router.navigate(["/"]);
      }
    });
  }

  failure(err: HttpErrorResponse) {
    if (err.status === 404) {
      this.snackbar.open("user does not exist", "Dismiss", {
        duration: 3000
      });
    }
  }
}
