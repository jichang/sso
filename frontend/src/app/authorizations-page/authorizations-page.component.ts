import { Component, OnInit, OnDestroy } from "@angular/core";
import {
  Authorization,
  AuthorizationModelService
} from "../authorization-model.service";
import { session } from "../model";
import { Subscription } from "rxjs";
import { Router } from "@angular/router";
import { ConfirmDialogComponent } from "../confirm-dialog/confirm-dialog.component";
import { MatDialog, MatDialogRef } from "@angular/material";
import { AuthorizationAction } from "../authorizations-list/authorizations-list.component";

@Component({
  selector: "authorizations-page",
  templateUrl: "./authorizations-page.component.html",
  styleUrls: ["./authorizations-page.component.css"]
})
export class AuthorizationsPageComponent implements OnInit, OnDestroy {
  authorizations: Authorization[] = [];
  subscription: Subscription;
  dialogRef: MatDialogRef<ConfirmDialogComponent>;

  constructor(
    private router: Router,
    private authorizationModel: AuthorizationModelService,
    public dialog: MatDialog
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

  handleAction($event: AuthorizationAction) {
    let { type, authorization } = $event;

    switch (type) {
      case "revoke":
        this.dialogRef = this.dialog.open(ConfirmDialogComponent, {
          height: "400px",
          width: "600px",
          data: {
            title: "Delete Authorization?",
            message: "delete contact " + authorization.server_app.name
          }
        });
        this.dialogRef.afterClosed().subscribe(result => {
          if (result) {
            this.authorizationModel
              .remove(authorization)
              .subscribe((authorization: Authorization) => {
                console.log(authorization);
              });
          }
        });
        break;
    }
  }
}
