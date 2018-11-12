import { Component, OnInit } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import {
  Application,
  ApplicationModelService
} from "../application-model.service";
import { map } from "rxjs/operators";
import { session } from "../model";
import { Subscription } from "rxjs";
import { MatDialogRef, MatDialog } from "@angular/material";
import { ConfirmDialogComponent } from "../confirm-dialog/confirm-dialog.component";

@Component({
  selector: "application-page",
  templateUrl: "./application-page.component.html",
  styleUrls: ["./application-page.component.css"]
})
export class ApplicationPageComponent implements OnInit {
  application: Application = null;
  subscription: Subscription = null;
  dialogRef: MatDialogRef<ConfirmDialogComponent>;

  constructor(
    private route: ActivatedRoute,
    private router: Router,
    private applicationModel: ApplicationModelService,
    public dialog: MatDialog
  ) {}

  ngOnInit() {
    this.subscription = this.applicationModel.applications
      .pipe(
        map(applications =>
          applications.find(
            application =>
              application.id === parseInt(this.route.snapshot.params["id"])
          )
        )
      )
      .subscribe(application => {
        this.application = application;
      });

    let currUser = session.currUser();
    if (currUser) {
      this.applicationModel.select(currUser.id);
    } else {
      this.router.navigate(["login"]);
    }
  }

  remove(application: Application) {
    this.dialogRef = this.dialog.open(ConfirmDialogComponent, {
      height: "400px",
      width: "600px",
      data: {
        title: "Delete Application?",
        message: "delete application " + application.name
      }
    });
    this.dialogRef.afterClosed().subscribe(result => {
      if (result) {
        this.applicationModel
          .remove(application)
          .subscribe((application: Application) => {
            this.router.navigate([".."], {
              relativeTo: this.route
            });
          });
      }
    });
  }
}
