import { Component, OnInit, OnDestroy } from "@angular/core";
import {
  Authorization,
  AuthorizationModelService
} from "../authorization-model.service";
import { session } from "../model";
import { Subscription } from "rxjs";
import { Router } from "@angular/router";

@Component({
  selector: "authorizations-page",
  templateUrl: "./authorizations-page.component.html",
  styleUrls: ["./authorizations-page.component.css"]
})
export class AuthorizationsPageComponent implements OnInit, OnDestroy {
  authorizations: Authorization[] = [];
  subscription: Subscription;

  constructor(
    private router: Router,
    private authorizationModel: AuthorizationModelService
  ) {}

  ngOnInit() {
    this.subscription = this.authorizationModel.authorizations.subscribe(
      authorizations => {
        this.authorizations = authorizations;
      }
    );

    let currUser = session.currUser();
    if (currUser) {
      this.authorizationModel.select(currUser.id);
    } else {
      this.router.navigate(["login"]);
    }
  }

  ngOnDestroy() {
    this.subscription.unsubscribe();
  }

  openCreateModal() {}
}
