import { Component, OnInit } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import * as model from "../model";
import { HttpErrorResponse } from "@angular/common/http";
import { MatSnackBar } from "@angular/material";

export enum AuthState {
  PASS = 0,
  NEED_TOTP = 1,
  NEED_WEBAUTHN = 2
}

export enum SigninStep {
  PASSWORD,
  TOTP,
  WEBAUTHN
}

@Component({
  selector: "signin-page",
  templateUrl: "./signin-page.component.html",
  styleUrls: ["./signin-page.component.css"]
})
export class SigninPageComponent implements OnInit {
  SigninStep = SigninStep;
  step: SigninStep = SigninStep.PASSWORD;

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private snackbar: MatSnackBar
  ) {}

  ngOnInit() {}

  passwordPassed({
    state,
    jwt,
    user
  }: {
    state: AuthState;
    jwt: string;
    user: model.User;
  }) {
    switch (state) {
      case AuthState.PASS:
      case AuthState.NEED_WEBAUTHN:
        this.signined({
          jwt,
          user
        });
        break;
      case AuthState.NEED_TOTP:
        this.step = SigninStep.TOTP;
        break;
    }
  }

  totpPassed({ jwt, user }: { jwt: string; user: model.User }) {
    this.signined({
      jwt,
      user
    });
  }

  signined({ jwt, user }: { jwt: string; user: model.User }) {
    window.localStorage.setItem("jwt", jwt);
    window.localStorage.setItem("currUser", JSON.stringify(user));

    this.route.queryParamMap.subscribe(params => {
      const redirectUrl = params.get("redirectUrl");
      if (redirectUrl) {
        window.location.href = decodeURIComponent(redirectUrl);
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

    if (err.status === 400) {
      this.snackbar.open("invalid parameter", "Dismiss", {
        duration: 3000
      });
    }

    if (err.status === 403) {
      this.snackbar.open("action is forbidden", "Dismiss", {
        duration: 3000
      });
    }
  }
}
