import { Component, OnInit, OnDestroy } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import {
  Application,
  ApplicationModelService
} from "../application-model.service";
import { map } from "rxjs/operators";
import { Subscription } from "rxjs";
import { MatDialogRef, MatDialog } from "@angular/material";
import { ConfirmDialogComponent } from "../confirm-dialog/confirm-dialog.component";
import { ScopeModelService, Scope } from "../scope-model.service";
import { SessionService } from "../session.service";

@Component({
  selector: "application-page",
  templateUrl: "./application-page.component.html",
  styleUrls: ["./application-page.component.css"]
})
export class ApplicationPageComponent implements OnInit, OnDestroy {
  application: Application = null;
  scopes: Scope[] = [];
  subscription: Subscription = null;
  dialogRef: MatDialogRef<ConfirmDialogComponent>;

  constructor(
    private session: SessionService,
    private route: ActivatedRoute,
    private router: Router,
    private applicationModel: ApplicationModelService,
    private scopeModel: ScopeModelService,
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

    this.scopeModel.scopes.subscribe(scopes => {
      this.scopes = scopes;
    });

    let current = this.session.current();
    if (current) {
      const currUser = current.currUser;
      this.applicationModel.select(currUser.id);
      this.scopeModel.select(currUser.id, this.route.snapshot.params["id"]);
    } else {
      this.router.navigate(["signin"]);
    }
  }

  ngOnDestroy() {
    this.subscription.unsubscribe();
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
